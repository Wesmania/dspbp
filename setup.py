from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="dspbp",
    version="0.0.3",
    rust_extensions=[RustExtension("dspbp.dspbp", binding=Binding.PyO3)],
    packages=["dspbp"],
    zip_safe=False,
)
