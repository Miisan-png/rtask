import os
import shutil
import platform
import subprocess


#cooked util file for reset (testing only LOL)
def get_home_directory():
    return os.path.expanduser("~")

def uninstall_rtask_cargo():
    try:
        subprocess.run(["cargo", "uninstall", "rtask"], check=False)
    except Exception as e:
        print(e)

def delete_rtask_files():
    home_dir = get_home_directory()
    rtasks_dir = os.path.join(home_dir, ".rtasks")
    confy_dir = os.path.join(home_dir, ".config", "rtask")
    if os.path.exists(rtasks_dir):
        try:
            shutil.rmtree(rtasks_dir)
        except Exception as e:
            print(e)

    if os.path.exists(confy_dir):
        try:
            shutil.rmtree(confy_dir)
        except Exception as e:
            print(e)

    
    app_data_local = ""
    
    if platform.system() == "Windows":
        app_data_local = os.path.join(os.environ.get("LOCALAPPDATA", ""), "rtask")
        app_data_roaming = os.path.join(os.environ.get("APPDATA", ""), "rtask")
        
        for path in [app_data_local, app_data_roaming]:
            if os.path.exists(path):
                try:
                    shutil.rmtree(path)
                except Exception as e:
                    print(e)

def main():
    uninstall_rtask_cargo()
    delete_rtask_files()

main()