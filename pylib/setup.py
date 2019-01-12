from setuptools import setup
from setuptools_rust import Binding, RustExtension
import toml

cfg = toml.load("Cargo.toml")
package_meta = cfg['package']

extensions = [
    RustExtension("rust_metropolis.string_sum", binding=Binding.PyO3)
]

setup(
    name=package_meta["name"],
    version=package_meta["version"],
    rust_extensions=extensions,
    packages=["rust_metropolis"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)