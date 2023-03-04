lines="""tests/uci_logs/berserk_w_game_1.txt
tests/uci_logs/berserk_b_game_1.txt
tests/uci_logs/sf_w_game_1.txt
tests/uci_logs/sf_b_game_1.txt
tests/uci_logs/sf_w_game_2.txt
tests/uci_logs/sf_b_game_2.txt
tests/uci_logs/sf_w_game_3.txt
tests/uci_logs/sf_b_game_3.txt
"""

for line in lines.splitlines():
    print("#[test]")
    print("fn roundtrip_" + line.replace("tests/uci_logs/", "").replace(".txt", "") + "() {")
    print(f"    roundtrip_logs(\"{line}\")")
    print("}")
    print()
