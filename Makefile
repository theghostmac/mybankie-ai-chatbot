.PHONY: venv install

venv:
	python3 -m venv venv

install: venv
	. venv/bin/activate; uv install -r requirements.txt

run:
	. venv/bin/activate; python your_script.py
