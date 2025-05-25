#!/bin/python3
import os
import shutil
from pathlib import Path
from maketools.toolchains import Buildtools
import zipfile
from datetime import datetime

build_help_text = """\
python3 ./make.py build:
    --help:
        print this help
    --clean:
        clean up
    --check:
        run cargo check
    --release:
        release build
    --debug:
        debug build
    --nightly:
        Introducing more optimizations using rust nightly
    --verbose:
        print details of build\
"""
CFLAGS = (
    "-Ofast -flto -fmerge-all-constants -fno-exceptions -fomit-frame-pointer -fshort-enums \
-Wl,-O3,--lto-O3,--gc-sections,--as-needed,--icf=all,-z,norelro,--pack-dyn-relocs=android+relr \
-std=c++2b -Wall -lc++"
)


def __parse_args(args):
    check = False
    release = False
    debug = False
    build = False
    verbose = False
    clean = False
    nightly = False

    for arg in args:
        match arg:
            case "--release" | "-r":
                release = True
                build = True
            case "--debug" | "-d":
                debug = True
                build = True
            case "--clean":
                clean = True
            case "--nightly":
                nightly = True
            case "--verbose" | "verbose" | "-v":
                verbose = True
            case "--check":
                check = True
                build = False
            case "-h" | "--help":
                print(build_help_text)
            case _:
                raise Exception("Illegal build parameter: {}".format(arg))

    if not build and not clean and not check:
        raise Exception(
            "Missing necessary build task argument(--release / --debug / --clean)"
        )
    elif (release and debug) or (build and clean) or (check and clean):
        raise Exception("Conflicting build arguments")

    return (check, clean, release, nightly, verbose)


def __clean():
    try:
        shutil.rmtree("output")
    except Exception:
        pass

    os.system("cargo clean")


def task(args):
    os.putenv("CARGO_CFG_BPF_TARGET_ARCH", "aarch64")

    try:
        tools = Buildtools()
    except Exception as err:
        exit(-1)

    try:
        (check, clean, release, nightly, verbose) = __parse_args(args)
    except Exception as err:
        exit(-1)

    if clean:
        __clean()
        return

    try:
        Path("output").mkdir()
    except Exception:
        pass

    if release:
        temp_dir = Path("output").joinpath(".temp").joinpath("release")
    else:
        temp_dir = Path("output").joinpath(".temp").joinpath("debug")

    try:
        shutil.rmtree(temp_dir)
    except Exception:
        pass

    if nightly:
        cargo = tools.cargo_nightly()
    else:
        cargo = tools.cargo()

    if check:
        cargo.arg("check --target aarch64-linux-android")
    else:
        cargo.arg("build --target aarch64-linux-android")
        if nightly:
            cargo.arg("-Z trim-paths")
            if release:
                cargo.arg("-Z build-std")
            

    if release:
        cargo.arg("--release")
    if verbose:
        cargo.arg("--verbose")

    # cargo.rust_flag("-C default-linker-libraries")
    cargo.build()

    if check:
        print("Finish check")
        return

    module = Path("module")
    shutil.copytree(module, temp_dir)
    temp_dir.joinpath(".gitignore").unlink()
    bin = Path("target").joinpath("aarch64-linux-android")
    if release:
        bin = bin.joinpath("release")
    else:
        bin = bin.joinpath("debug")
    bin = bin.joinpath("scx_controller")

    bin_module = temp_dir.joinpath("scx_controller")
    shutil.copy2(bin, bin_module)
    tools.strip(bin_module)

    build_time = datetime.now().strftime("%Y-%m-%d-%Hh%Mm%Ss")
    build_type = "release" if release else "debug"
    output = Path("output") / f"scx_controller_{build_type}_{build_time}"

    with zipfile.ZipFile(
        f"{output}.zip", "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9
    ) as zipf:
        for root, _, files in os.walk(temp_dir):
            for file in files:
                filepath = os.path.join(root, file)
                arcname = os.path.relpath(filepath, temp_dir)
                zipf.write(filepath, arcname)
    print("scx_controller build successfully: {}.zip".format(output))
