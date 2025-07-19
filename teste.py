# String hexadecimal
hex_data = "45a4d25992d6ad43c024c09e0c000000c652200f00000000a199f90e0000000001000000c024c09e0c0000000100000001000000010000002e010000006440420f00000000000032b6010000000000 "

# Converte a string hexadecimal em um array de bytes
byte_array = bytes.fromhex(hex_data)

# Converte os bytes para uma lista de inteiros decimais
decimal_values = list(byte_array)

print(decimal_values)
