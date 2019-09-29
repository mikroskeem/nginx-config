#!/usr/bin/env python3

import argparse
import json
#import jinja2

templates = {
    "le_reverse_proxy": {
        "description": "Generates a reverse proxy configuration",
        "template_file": "le_reverse_proxy.conf",
        "options": [
            {"key": "domain_name", "description": "Domain name"},
            {"key": "reverse_proxy_target", "description": "Reverse proxy target"},
            {"key": "proxy_websockets", "description": "Whether to support websocket proxying or not", "default": True}
        ]
    },
    "le_php_fpm": {
        "description": "Generates a PHP configuration suitable for various PHP web applications",
        "template_file": "le_php.conf",
        "options": [
            {"key": "domain_name", "description": "Domain name"},
            {"key": "spa", "description": "Whether web app is a SPA (Single Page App, such as Laravel etc. based projects)", "default": False}
        ]
    }
}

parser = argparse.ArgumentParser(description="Generate a nginx configuration based on templates and options", allow_abbrev=False)
parser.add_argument("--template", type=str, help="Picks a template")
parser.add_argument("-l", "--list-templates", action="store_true", help="Lists available templates")

args = parser.parse_args()

if args.list_templates:
    print(">>> Templates:")
    for template_id, template_data in templates.items():
        print(f"- {template_id}")
        print(f"\t{template_data['description']}")
        print("\tOptions:")
        for option in template_data["options"]:
            print(f"\t\t{option['key']} - {option['description']}")
            if "default" in option:
                print(f"\t\t\tDefault value: {option['default']}")
            else:
                print(f"\t\t\tREQUIRED")
    exit(0)

if args.template:
    print("Not done yet :(")
else:
    parser.print_help()
