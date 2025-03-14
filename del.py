#!/usr/bin/env python3
"""
Script to delete all RTask data and configuration files
"""

import os
import shutil
import platform
import subprocess

def get_home_directory():
    """Get the user's home directory"""
    return os.path.expanduser("~")

def uninstall_rtask_cargo():
    """Uninstall rtask using cargo"""
    try:
        print("Attempting to uninstall rtask from cargo...")
        subprocess.run(["cargo", "uninstall", "rtask"], check=False)
        print("Cargo uninstall command completed.")
    except Exception as e:
        print(f"Error during cargo uninstall: {e}")

def delete_rtask_files():
    """Delete RTask configuration and data files"""
    home_dir = get_home_directory()
    
    # Paths to delete
    rtasks_dir = os.path.join(home_dir, ".rtasks")
    confy_dir = os.path.join(home_dir, ".config", "rtask")
    
    # Delete .rtasks directory
    if os.path.exists(rtasks_dir):
        print(f"Deleting {rtasks_dir}...")
        try:
            shutil.rmtree(rtasks_dir)
            print(f"✓ Deleted {rtasks_dir}")
        except Exception as e:
            print(f"Error deleting {rtasks_dir}: {e}")
    else:
        print(f"{rtasks_dir} not found")
    
    # Delete config directory
    if os.path.exists(confy_dir):
        print(f"Deleting {confy_dir}...")
        try:
            shutil.rmtree(confy_dir)
            print(f"✓ Deleted {confy_dir}")
        except Exception as e:
            print(f"Error deleting {confy_dir}: {e}")
    else:
        print(f"{confy_dir} not found")
    
    # Check additional possible locations
    app_data_local = ""
    
    if platform.system() == "Windows":
        # Check AppData locations on Windows
        app_data_local = os.path.join(os.environ.get("LOCALAPPDATA", ""), "rtask")
        app_data_roaming = os.path.join(os.environ.get("APPDATA", ""), "rtask")
        
        for path in [app_data_local, app_data_roaming]:
            if os.path.exists(path):
                print(f"Deleting {path}...")
                try:
                    shutil.rmtree(path)
                    print(f"✓ Deleted {path}")
                except Exception as e:
                    print(f"Error deleting {path}: {e}")

def main():
    print("RTask Data Deletion Utility")
    print("==========================")
    
    # Uninstall from cargo
    uninstall_rtask_cargo()
    
    # Delete data files
    delete_rtask_files()
    
    print("\nDone! All RTask data and configuration files should be removed.")
    print("The next time you run rtask, it will start with a fresh setup.")

if __name__ == "__main__":
    main()