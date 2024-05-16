# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/components/draw_order.fbs".

# You can extend this class by creating a "DrawOrderExt" class in "draw_order_ext.py".

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field

from .._baseclasses import BaseBatch, BaseExtensionType, ComponentBatchMixin

__all__ = ["DrawOrder", "DrawOrderArrayLike", "DrawOrderBatch", "DrawOrderLike", "DrawOrderType"]


@define(init=False)
class DrawOrder:
    """
    **Component**: Draw order used for the display order of 2D elements.

    Higher values are drawn on top of lower values.
    An entity can have only a single draw order component.
    Within an entity draw order is governed by the order of the components.

    Draw order for entities with the same draw order is generally undefined.
    """

    def __init__(self: Any, value: DrawOrderLike):
        """Create a new instance of the DrawOrder component."""

        # You can define your own __init__ function as a member of DrawOrderExt in draw_order_ext.py
        self.__attrs_init__(value=value)

    value: float = field(converter=float)

    def __array__(self, dtype: npt.DTypeLike = None) -> npt.NDArray[Any]:
        # You can define your own __array__ function as a member of DrawOrderExt in draw_order_ext.py
        return np.asarray(self.value, dtype=dtype)

    def __float__(self) -> float:
        return float(self.value)

    def __hash__(self) -> int:
        return hash(self.value)


if TYPE_CHECKING:
    DrawOrderLike = Union[DrawOrder, float]
else:
    DrawOrderLike = Any

DrawOrderArrayLike = Union[DrawOrder, Sequence[DrawOrderLike], float, npt.NDArray[np.float32]]


class DrawOrderType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.components.DrawOrder"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.float32(), self._TYPE_NAME)


class DrawOrderBatch(BaseBatch[DrawOrderArrayLike], ComponentBatchMixin):
    _ARROW_TYPE = DrawOrderType()

    @staticmethod
    def _native_to_pa_array(data: DrawOrderArrayLike, data_type: pa.DataType) -> pa.Array:
        array = np.asarray(data, dtype=np.float32).flatten()
        return pa.array(array, type=data_type)
