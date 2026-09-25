"""Prueba de interfaz; requiere TenSEAL instalado explícitamente."""
try:
    import tenseal as ts
except ImportError as exc:
    raise SystemExit("Instala TenSEAL solo en el entorno de laboratorio: pip install tenseal") from exc

ctx = ts.context(ts.SCHEME_TYPE.CKKS, poly_modulus_degree=8192, coeff_mod_bit_sizes=[60, 40, 40, 60])
ctx.global_scale = 2**40
ctx.generate_galois_keys()
encrypted = ts.ckks_vector(ctx, [1.0, 2.0])
print("demo suma cifrada:", encrypted.decrypt()[0] + encrypted.decrypt()[1])
