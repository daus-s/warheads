import json
import os
from dotenv import load_dotenv


class bcolors:
    RED = "\033[31m"
    GREEN = "\033[32m"
    ENDC = "\033[0m"
    BOLD = "\033[1m"
    UNDERLINE = "\033[4m"


def needs_date_correction(path):
    try:
        with open(path, "r") as file:
            data = json.load(file)

            date = data["game_date"]

            return "/" in date

    except FileNotFoundError:
        print(f"Error: file not found: {path}")
    except json.JSONDecodeError:
        print(f"Error: Could not decode JSON from file {path}.")


def correct_date_format(path):
    try:
        with open(path, "r+") as file:
            data = json.load(file)

            date = data["game_date"]

            if "/" in date:
                date = date.split("/")
                [m, d, y] = (date[0], date[1], date[2])

                data["game_date"] = f"{y}-{m}-{d}"

                _ = file.seek(0)
                json.dump(data, file, indent=2)
                _ = file.truncate()

                print(f"updated {path}")

    except FileNotFoundError:
        print(f"Error: file not found: {path}")
    except json.JSONDecodeError:
        print(f"Error: Could not decode JSON from file {path}.")


def update_correction_dates():
    dir = os.getenv("DATA")

    if dir is None:
        print("Error: DATA environment variable not set.")
        return

    dir = os.path.join(dir, "nba", "corrections")

    for root, _dirs, files in os.walk(dir):
        path = root.replace(dir, "").split(os.sep)
        print((len(path) - 1) * "---", os.path.basename(root))
        for file in files:
            if file.endswith(".corr"):
                if needs_date_correction(os.path.join(root, file)):
                    print("❌" + len(path) * "---" + bcolors.RED + file + bcolors.ENDC)

                    correct_date_format(os.path.join(root, file))
                else:
                    print(
                        "✅" + len(path) * "---" + bcolors.GREEN + file + bcolors.ENDC
                    )
            else:
                print(len(path) * "---", file)

        # if file.endswith(".json"):
        #     correct_date_format(os.path.join(root, file))
    # check


def main():
    update_correction_dates()


if __name__ == "__main__":
    load_dotenv()
    main()
