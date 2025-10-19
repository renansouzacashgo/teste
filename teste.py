# String hexadecimal
hex_data = "faea0d7bd59c13ec40420f0000000000b9ffdd28010000000000000000000000"

# Converte a string hexadecimal em um array de bytes
byte_array = bytes.fromhex(hex_data)

# Converte os bytes para uma lista de inteiros decimais
decimal_values = list(byte_array)

print(decimal_values)
