from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    author="Wesmania",
    author_email="ikotrasinsk@gmail.com",
    rust_extensions=[RustExtension("dspbp", binding=Binding.PyO3, features=["python"])],
    zip_safe=False,
)
