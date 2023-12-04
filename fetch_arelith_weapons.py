import requests
import sys
import rich
from bs4 import BeautifulSoup

URL = "https://wiki.nwnarelith.com/Weapons"
OUTPUT_FILE_PATH = "./src/simulator/item/weapon_db.rs"
TAB_SPACE = " " * 4

def get_rust_code(weapon_name, size, damage, threat_range, crit_multiplier, damage_type_list):
    code  = ""
    code += "{tab_1}(\n"
    code +=     "{tab_2}\"{weapon_name}\".into(),\n"
    code +=     "{tab_2}WeaponBase::new(\n"
    code +=         "{tab_3}\"{weapon_name}\".into(),\n"
    code +=         "{tab_3}SizeCategory::{size},\n"
    code +=         "{tab_3}Dice::from(\"{damage}\"),\n"
    code +=         "{tab_3}{threat_range},\n"
    code +=         "{tab_3}{crit_multiplier},\n"
    code +=         "{tab_3}vec![{damage_types}],\n"
    code +=     "{tab_2}),\n"
    code += "{tab_1}),\n"

    code = code.format(tab_1 = TAB_SPACE * 2, 
                       tab_2 = TAB_SPACE * 3, 
                       tab_3 = TAB_SPACE * 4, 
                       weapon_name = weapon_name,
                       size = size,
                       damage = damage,
                       threat_range = threat_range,
                       crit_multiplier = crit_multiplier,
                       damage_types = ", ".join(["DamageType::" + elem for elem in damage_type_list]))

    return code

html = requests.get(URL).content
bs = BeautifulSoup(html, "html.parser")

tr_list = bs.select("table")[-1].select("tr")
file_lines = None
file_gen_start_index = None
file_gen_end_index = None

with open(OUTPUT_FILE_PATH, "r+") as f:
    file_lines = f.readlines()

    for i, line in enumerate(file_lines):
        if "//--AUTO-GENERATION-START" in line:
            file_gen_start_index = i
        elif "//--AUTO-GENERATION-END" in line:
            file_gen_end_index = i

if file_gen_start_index is None \
or file_gen_end_index is None:
    print("[!] Failed to export weapons: ile_gen_start_index or file_gen_end_index is None")
    sys.exit()

file_lines = [elem for i, elem in enumerate(file_lines) if i <= file_gen_start_index or i >= file_gen_end_index]

for tr in tr_list:
    td_list = tr.select("td")
    if len(td_list) < 1: continue

    weapon_name = td_list[0].text.strip()
    size = td_list[2].text.strip().upper()
    damage = td_list[3].text.strip()
    critical = td_list[4].text.strip()
    damage_type = td_list[5].text.strip()

    # fix formatings
    weapon_name = weapon_name.replace("(", "").replace(")", "")
    
    if weapon_name == "Mace Light Mace":
        weapon_name = "Light Mace"
    elif weapon_name == "Lance Small":
        weapon_name = "Small Lance"

    if size == "T":
        size = "Tiny"
    elif size == "S":
        size = "Small"
    elif size == "M":
        size = "Medium"
    elif size == "L":
        size = "Large"
    elif size == "H":
        size = "Huge"

    damage = damage.split("/")[0]
    critical_split = [elem for elem in critical.split("x") if elem]
    threat_range = critical_split[0].replace("/", "").split("-")[0] if len(critical_split) > 1 else 20
    crit_multiplier = critical_split[1] if len(critical_split) > 1 else critical_split[0]

    damage_type_list = []
    damage_type_split = damage_type.split("-")

    for elem in damage_type_split:
        if elem in "Bludgeoning":
            damage_type_list.append("Bludgeoning")
        elif elem in "Slashing":
            damage_type_list.append("Slashing")
        elif elem in "Pierce" or elem in "Piercing":
            damage_type_list.append("Piercing")

    # print(weapon_name)
    # print(size)
    # print(damage)
    # print(threat_range)
    # print(crit_multiplier)
    # print(", ".join(damage_type_list))
    # print("-"*10)

    file_lines.insert(file_gen_start_index + 1, get_rust_code(weapon_name, size, damage, threat_range, crit_multiplier, damage_type_list))

with open(OUTPUT_FILE_PATH, "w") as f:
    f.writelines(file_lines)
