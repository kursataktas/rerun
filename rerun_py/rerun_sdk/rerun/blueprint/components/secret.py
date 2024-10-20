# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/blueprint/components/secrets.fbs".

# You can extend this class by creating a "SecretExt" class in "secret_ext.py".

from __future__ import annotations

from ... import datatypes
from ..._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

__all__ = ["Secret", "SecretBatch", "SecretType"]


class Secret(datatypes.Utf8, ComponentMixin):
    """**Component**: String type to hold a secret value."""

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of SecretExt in secret_ext.py

    # Note: there are no fields here because Secret delegates to datatypes.Utf8
    pass


class SecretType(datatypes.Utf8Type):
    _TYPE_NAME: str = "rerun.blueprint.components.Secret"


class SecretBatch(datatypes.Utf8Batch, ComponentBatchMixin):
    _ARROW_TYPE = SecretType()


# This is patched in late to avoid circular dependencies.
Secret._BATCH_TYPE = SecretBatch  # type: ignore[assignment]
