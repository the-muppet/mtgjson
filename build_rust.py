#!/usr/bin/env python3
"""
Build script for the mtgjson-rust Python extension module.
This script handles building the Rust module locally for development.
"""

import os
import subprocess
import sys
import shutil
from pathlib import Path

def check_rust_installed():
    """Check if Rust is installed on the system."""
    try:
        result = subprocess.run(['rustc', '--version'], capture_output=True, text=True)
        print(f"✓ Rust found: {result.stdout.strip()}")
        return True
    except FileNotFoundError:
        print("✗ Rust not found. Please install Rust from https://rustup.rs/")
        return False

def check_maturin_installed():
    """Check if maturin is installed and install it if not."""
    try:
        result = subprocess.run(['maturin', '--version'], capture_output=True, text=True)
        print(f"✓ Maturin found: {result.stdout.strip()}")
        return True
    except FileNotFoundError:
        print("✗ Maturin not found. Installing...")
        try:
            subprocess.run([sys.executable, '-m', 'pip', 'install', 'maturin'], check=True)
            print("✓ Maturin installed successfully")
            return True
        except subprocess.CalledProcessError:
            print("✗ Failed to install maturin")
            return False

def build_rust_module(mode='release'):
    """Build the Rust module using maturin."""
    # Get the absolute path to the script directory
    script_dir = Path(__file__).parent.absolute()
    rust_dir = script_dir / 'mtgjson-rust'
    
    if not rust_dir.exists():
        print(f"✗ Rust directory {rust_dir} not found")
        print(f"  Current working directory: {os.getcwd()}")
        print(f"  Script directory: {script_dir}")
        print(f"  Looking for: {rust_dir}")
        print(f"  Available directories in {script_dir}:")
        try:
            for item in script_dir.iterdir():
                if item.is_dir():
                    print(f"    - {item.name}")
        except Exception as e:
            print(f"    Error listing directories: {e}")
        return False
    
    # Validate Cargo.toml exists
    cargo_toml = rust_dir / 'Cargo.toml'
    if not cargo_toml.exists():
        print(f"✗ Cargo.toml not found at {cargo_toml}")
        return False
    
    print(f"Building Rust module in {mode} mode...")
    print(f"  Rust directory: {rust_dir}")
    print(f"  Cargo.toml: {cargo_toml}")
    
    # Change to the Rust directory
    original_dir = os.getcwd()
    os.chdir(rust_dir)
    
    try:
        # Build command
        cmd = ['maturin', 'develop']
        if mode == 'release':
            cmd.append('--release')
        
        result = subprocess.run(cmd, check=True)
        print("✓ Rust module built and installed successfully")
        return True
        
    except subprocess.CalledProcessError as e:
        print(f"✗ Failed to build Rust module: {e}")
        return False
    finally:
        os.chdir(original_dir)

def build_wheel():
    """Build a wheel for distribution."""
    # Get the absolute path to the script directory
    script_dir = Path(__file__).parent.absolute()
    rust_dir = script_dir / 'mtgjson-rust'
    
    if not rust_dir.exists():
        print(f"✗ Rust directory {rust_dir} not found")
        print(f"  Current working directory: {os.getcwd()}")
        print(f"  Script directory: {script_dir}")
        print(f"  Looking for: {rust_dir}")
        print(f"  Available directories in {script_dir}:")
        try:
            for item in script_dir.iterdir():
                if item.is_dir():
                    print(f"    - {item.name}")
        except Exception as e:
            print(f"    Error listing directories: {e}")
        return False
    
    # Validate Cargo.toml exists
    cargo_toml = rust_dir / 'Cargo.toml'
    if not cargo_toml.exists():
        print(f"✗ Cargo.toml not found at {cargo_toml}")
        return False
    
    print("Building wheel...")
    print(f"  Rust directory: {rust_dir}")
    print(f"  Cargo.toml: {cargo_toml}")
    
    # Change to the Rust directory
    original_dir = os.getcwd()
    os.chdir(rust_dir)
    
    try:
        # Build wheel
        result = subprocess.run(['maturin', 'build', '--release'], check=True)
        print("✓ Wheel built successfully")
        
        # Find and copy the wheel to the project root
        target_dir = Path('target/wheels')
        if target_dir.exists():
            wheels = list(target_dir.glob('*.whl'))
            if wheels:
                latest_wheel = max(wheels, key=lambda p: p.stat().st_mtime)
                shutil.copy(latest_wheel, '../')
                print(f"✓ Wheel copied to project root: {latest_wheel.name}")
        
        return True
        
    except subprocess.CalledProcessError as e:
        print(f"✗ Failed to build wheel: {e}")
        return False
    finally:
        os.chdir(original_dir)

def print_troubleshooting_info():
    """Print troubleshooting information."""
    print("\n" + "=" * 50)
    print("TROUBLESHOOTING INFORMATION")
    print("=" * 50)
    print(f"Platform: {sys.platform}")
    print(f"Python: {sys.version}")
    print(f"Current directory: {os.getcwd()}")
    print(f"Script location: {Path(__file__).absolute()}")
    print(f"Script directory: {Path(__file__).parent.absolute()}")
    
    script_dir = Path(__file__).parent.absolute()
    print(f"\nContents of {script_dir}:")
    try:
        for item in script_dir.iterdir():
            print(f"  {'[DIR]' if item.is_dir() else '[FILE]'} {item.name}")
    except Exception as e:
        print(f"  Error listing contents: {e}")
    
    if sys.platform == "win32":
        print("\nWindows-specific notes:")
        print("- Make sure you're running from the correct directory")
        print("- Try using forward slashes or raw strings for paths")
        print("- Ensure PowerShell/Command Prompt has proper permissions")

def main():
    """Main function."""
    import argparse
    
    parser = argparse.ArgumentParser(description='Build the mtgjson-rust Python extension')
    parser.add_argument('--mode', choices=['debug', 'release'], default='release',
                       help='Build mode (default: release)')
    parser.add_argument('--wheel', action='store_true',
                       help='Build a wheel instead of installing in development mode')
    parser.add_argument('--check-only', action='store_true',
                       help='Only check if required tools are installed')
    parser.add_argument('--troubleshoot', action='store_true',
                       help='Print troubleshooting information and exit')
    
    args = parser.parse_args()
    
    if args.troubleshoot:
        print_troubleshooting_info()
        return
    
    print("MTGJSON Rust Module Builder")
    print("=" * 30)
    
    # Check prerequisites
    if not check_rust_installed():
        sys.exit(1)
    
    if not check_maturin_installed():
        sys.exit(1)
    
    if args.check_only:
        print("\n✓ All required tools are available")
        return
    
    # Build
    if args.wheel:
        success = build_wheel()
    else:
        success = build_rust_module(args.mode)
    
    if success:
        print("\n✓ Build completed successfully!")
        if not args.wheel:
            print("The mtgjson_rust module is now available for import in Python.")
    else:
        print("\n✗ Build failed!")
        print("\nFor troubleshooting information, run:")
        print(f"  python {Path(__file__).name} --troubleshoot")
        if sys.platform == "win32":
            print("\nCommon Windows issues:")
            print("- Make sure you're in the correct directory (same as this script)")
            print("- Check that the mtgjson-rust folder exists alongside this script")
            print("- Try running as Administrator if you get permission errors")
        sys.exit(1)

if __name__ == '__main__':
    main()