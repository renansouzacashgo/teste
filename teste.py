# String hexadecimal
hex_data = "33e685a4017f83ad0009ed7a480000006d3aea3801000000"

# Converte a string hexadecimal em um array de bytes
byte_array = bytes.fromhex(hex_data)

# Converte os bytes para uma lista de inteiros decimais
decimal_values = list(byte_array)

print(decimal_values)
