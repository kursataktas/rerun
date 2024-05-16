# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/blueprint/components/column_share.fbs".

# You can extend this class by creating a "ColumnShareExt" class in "column_share_ext.py".

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field

from ..._baseclasses import BaseBatch, BaseExtensionType, ComponentBatchMixin

__all__ = ["ColumnShare", "ColumnShareArrayLike", "ColumnShareBatch", "ColumnShareLike", "ColumnShareType"]


@define(init=False)
class ColumnShare:
    """**Component**: The layout share of a column in the container."""

    def __init__(self: Any, share: ColumnShareLike):
        """
        Create a new instance of the ColumnShare component.

        Parameters
        ----------
        share:
            The layout shares of a column in the container.

        """

        # You can define your own __init__ function as a member of ColumnShareExt in column_share_ext.py
        self.__attrs_init__(share=share)

    share: float = field(converter=float)
    # The layout shares of a column in the container.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    def __array__(self, dtype: npt.DTypeLike = None) -> npt.NDArray[Any]:
        # You can define your own __array__ function as a member of ColumnShareExt in column_share_ext.py
        return np.asarray(self.share, dtype=dtype)

    def __float__(self) -> float:
        return float(self.share)

    def __hash__(self) -> int:
        return hash(self.share)


if TYPE_CHECKING:
    ColumnShareLike = Union[ColumnShare, float]
else:
    ColumnShareLike = Any

ColumnShareArrayLike = Union[ColumnShare, Sequence[ColumnShareLike], npt.ArrayLike]


class ColumnShareType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.blueprint.components.ColumnShare"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.float32(), self._TYPE_NAME)


class ColumnShareBatch(BaseBatch[ColumnShareArrayLike], ComponentBatchMixin):
    _ARROW_TYPE = ColumnShareType()

    @staticmethod
    def _native_to_pa_array(data: ColumnShareArrayLike, data_type: pa.DataType) -> pa.Array:
        array = np.asarray(data, dtype=np.float32).flatten()
        return pa.array(array, type=data_type)
