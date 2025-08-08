import ctypes
import os
import platform

# Choose the DLL path according to your build/architecture
root = os.path.dirname(__file__)
if platform.architecture()[0] == '64bit':
    dll_path = os.path.join(root, 'target', 'x86_64-pc-windows-msvc', 'release', 'rust_dll.dll')
else:
    dll_path = os.path.join(root, 'target', 'i686-pc-windows-msvc', 'release', 'rust_dll.dll')

# Load with WinDLL to match stdcall on 32-bit and default on 64-bit
lib = ctypes.WinDLL(dll_path)

# Prototypes: return pointers (void*) so we can free them later
lib.developer.restype = ctypes.c_void_p
lib.developer.argtypes = []

lib.date.restype = ctypes.c_void_p
lib.date.argtypes = []

lib.concatenate.restype = ctypes.c_void_p
lib.concatenate.argtypes = [ctypes.c_char_p, ctypes.c_char_p]

lib.free_string.restype = None
lib.free_string.argtypes = [ctypes.c_void_p]


def take_string_and_free(ptr: int) -> str:
    """Convert a returned char* (UTF-8) to Python str and free it via free_string."""
    if not ptr:
        return ''
    # Read bytes from pointer
    raw = ctypes.cast(ptr, ctypes.c_char_p).value or b''
    # Free the original allocation
    lib.free_string(ctypes.c_void_p(ptr))
    # Decode UTF-8 with replacement for safety
    return raw.decode('utf-8', errors='replace')


# developer()
p_dev = lib.developer()
print('developer:', take_string_and_free(p_dev))

# date()
p_date = lib.date()
print('date:', take_string_and_free(p_date))

# concatenate("Olá", "Mundo") — pass UTF-8 encoded bytes
p_concat = lib.concatenate('Olá'.encode('utf-8'), 'Mundo'.encode('utf-8'))
print('concatenate:', take_string_and_free(p_concat))