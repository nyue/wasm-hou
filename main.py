import extism
import json

manifest = {"wasm": [
    {"url": "http://localhost:8000/wasm_sphere.wasm"}]}
with extism.Plugin(manifest, wasi=True) as plugin:
    wasm_vowel_count = plugin.call(
        'uv_sphere',
        '{"n_stacks":8,"n_slices":4}',
        parse=lambda output: json.loads(bytes(output).decode('utf-8'))
    )
print(wasm_vowel_count)  # {'count': 3, 'total': 3, 'vowels': 'aeiouAEIOU'}
