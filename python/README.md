```
# setup
$ poetry install
$ poetry run maturin build --release

# execute
$ swith to Python more than version 3.12
$ python -m venv YOUR_VENV_NAME
$ source ./YOUR_VENV_NAME/bin/activate
$ pip install ./target/wheels/multi_bind_py-XXX.whl
## or
$ pip install --force-reinstall ./target/wheels/multi_bind_py-XXX.whl

$ python
python> import multi_bind_py
python> multi_bind_py.add(1, 2)
>> 3
python> multi_bind_py.unwrap_future()
>> 10
```
