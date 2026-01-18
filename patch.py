"""Script to patch the LPC176x5x SVD."""

import logging
import re
import xml.etree.ElementTree as ET
from pathlib import Path
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from xml.dom.minidom import Element
    from xml.etree.ElementTree import ElementTree

INPUT_SVD_PATH: Path = Path(__file__).parent.joinpath("LPC176x5x_v0.2.svd")
OUTPUT_SVD_PATH: Path = Path(__file__).parent.joinpath("LPC176x5x_v0.2-python.svd")


logging.basicConfig(
    level=logging.INFO,
    format="[%(asctime)s][%(name)s][%(levelname)s] %(message)s",
    datefmt="%Y-%m-%d %H:%M:%S",
)
logger: logging.Logger = logging.getLogger("Main")


def clean_string(string: str) -> str:
    """Removes the literal substring '%s' and all non-word characters."""
    return re.sub(r"\W", "", string.replace("%s", ""))


def main() -> None:
    """Perform the SVD patch."""
    logger.info("Opening %s...", str(INPUT_SVD_PATH))
    tree: ElementTree[Element[str]] = ET.parse(INPUT_SVD_PATH)
    root: Element[str] = tree.getroot()
    count: int = 0
    for peripheral in root.findall(".//peripheral"):
        p_name_element: Element[str] | None = peripheral.find("name")
        if p_name_element is None or p_name_element.text is None:
            logger.warning("Could not find peripheral name")
            continue
        p_name = clean_string(p_name_element.text)
        for register in peripheral.findall(".//register"):
            r_name_element: Element[str] | None = register.find("name")
            if r_name_element is None or r_name_element.text is None:
                logger.warning("Could not find register name")
                continue
            r_name = clean_string(r_name_element.text)
            for field in register.findall(".//field"):
                f_name_element: Element[str] | None = field.find("name")
                if f_name_element is None or f_name_element.text is None:
                    logger.warning("Could not find field name")
                    continue
                f_name = clean_string(f_name_element.text)
                for enums in field.findall("enumeratedValues"):
                    name_tag = enums.find("name")
                    if name_tag is not None and name_tag.text == "ENUM":
                        new_name = f"{p_name}_{r_name}_{f_name}_Enum"
                        name_tag.text = new_name
                        count += 1

    tree.write(file_or_filename=OUTPUT_SVD_PATH, encoding="utf-8", xml_declaration=True)
    logger.info("Done! Updated %d enum names.", count)


if __name__ == "__main__":
    main()
