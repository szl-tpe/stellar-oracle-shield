import black
from stellar_contract_bindings.python import generate_binding
from stellar_contract_bindings.utils import get_specs_by_wasm_file

from pathlib import Path

import yaml

from stellar_oracle_shield_simulation_suite.simulator.config import Config

DEFAULT_CONFIG_PATH = "config.yaml"
DEFAULT_OUTPUT_DIR = Path("src/stellar_oracle_shield_client/bindings.py")


def generate_bindings():
    path = Path(DEFAULT_CONFIG_PATH).resolve()
    with path.open(encoding="utf-8") as file:
        data = yaml.safe_load(file)
        config = Config.model_validate(data.get("simulator") or {})

    specs = get_specs_by_wasm_file(str(config.contract.wasm_path))
    generated = generate_binding(specs, client_type="both")
    formatted = black.format_str(generated, mode=black.Mode())

    DEFAULT_OUTPUT_DIR.parent.mkdir(parents=True, exist_ok=True)
    DEFAULT_OUTPUT_DIR.write_text(formatted, encoding="utf-8")

    print(f"bindings generated at {DEFAULT_OUTPUT_DIR}")
