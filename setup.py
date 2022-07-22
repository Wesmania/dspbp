from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="dspbp",
    version="0.0.3",
    rust_extensions=[RustExtension("dspbp", binding=Binding.PyO3, features=["python"])],
    zip_safe=False,
    python_requires=">=3.7",
)
