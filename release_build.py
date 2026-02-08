import subprocess
from pathlib import Path
from zipfile import ZipFile, ZIP_DEFLATED
import tomllib

# build release
subprocess.check_call(["cargo", "build", "--release"])

# read Cargo.toml
with open("Cargo.toml", "rb") as f:
    cargo = tomllib.load(f)

bin_name = cargo["package"]["name"]

# detect target triple
rustc_info = subprocess.check_output(["rustc", "-vV"], text=True)
target = [l for l in rustc_info.splitlines() if l.startswith("host:")][0].split()[1]

exe_path = Path(f"target/release/{bin_name}.exe")
zip_name = f"{bin_name}-{target}.zip"

# create zip with exe at root
with ZipFile(zip_name, "w", ZIP_DEFLATED) as z:
    z.write(exe_path, arcname=f"{bin_name}.exe")

print("Created:", zip_name)
