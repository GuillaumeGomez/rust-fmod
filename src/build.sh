#!/bin/bash
rustc http_sender.rs && rustc main.rs -L . -o http_sender