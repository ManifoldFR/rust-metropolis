from setuptools import setup
from setuptools_rust import Binding, RustExtension
import toml

cfg = toml.load("Cargo.toml")
package_meta = cfg['package']

setup_requires = ["setuptools-rust>=0.10.1"]

extensions = [
    RustExtension("rust_metropolis.random_walk", binding=Binding.PyO3)
]

setup(
    name=package_meta["name"],
    version=package_meta["version"],
    classifiers=[
        "Programming Language :: Python",
        "Programming Language :: Rust",
    ],
    packages=["rust_metropolis"],
    rust_extensions=extensions,
    setup_requires=setup_requires,
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)
