# String hexadecimal
hex_data = "c1209b3341d69c8106020000000764000148640102d5f0561286000000d00869f303000000540155"

# Converte a string hexadecimal em um array de bytes
byte_array = bytes.fromhex(hex_data)

# Converte os bytes para uma lista de inteiros decimais
decimal_values = list(byte_array)

print(decimal_values)
