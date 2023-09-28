# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/archetypes/image.fbs".

# You can extend this class by creating a "ImageExt" class in "image_ext.py".

from __future__ import annotations

from typing import Any

from attrs import define, field

from .. import components, datatypes
from .._baseclasses import Archetype
from .image_ext import ImageExt

__all__ = ["Image"]


@define(str=False, repr=False, init=False)
class Image(ImageExt, Archetype):
    """
    A monochrome or color image.

    The shape of the `TensorData` must be mappable to:
    - A `HxW` tensor, treated as a grayscale image.
    - A `HxWx3` tensor, treated as an RGB image.
    - A `HxWx4` tensor, treated as an RGBA image.

    Leading and trailing unit-dimensions are ignored, so that
    `1x640x480x3x1` is treated as a `640x480x3` RGB image.

    Example
    -------
    ```python

    import numpy as np
    import rerun as rr

    # Create an image with numpy
    image = np.zeros((8, 12, 3), dtype=np.uint8)
    image[:, :, 0] = 255
    image[0:4, 0:6] = (0, 255, 0)

    rr.init("rerun_example_image_simple", spawn=True)

    rr.log("image", rr.Image(image))
    ```
    <picture>
      <source media="(max-width: 480px)" srcset="https://static.rerun.io/image_simple/06ba7f8582acc1ffb42a7fd0006fad7816f3e4e4/480w.png">
      <source media="(max-width: 768px)" srcset="https://static.rerun.io/image_simple/06ba7f8582acc1ffb42a7fd0006fad7816f3e4e4/768w.png">
      <source media="(max-width: 1024px)" srcset="https://static.rerun.io/image_simple/06ba7f8582acc1ffb42a7fd0006fad7816f3e4e4/1024w.png">
      <source media="(max-width: 1200px)" srcset="https://static.rerun.io/image_simple/06ba7f8582acc1ffb42a7fd0006fad7816f3e4e4/1200w.png">
      <img src="https://static.rerun.io/image_simple/06ba7f8582acc1ffb42a7fd0006fad7816f3e4e4/full.png">
    </picture>
    """

    def __init__(self: Any, data: datatypes.TensorDataLike, *, draw_order: components.DrawOrderLike | None = None):
        """
        Create a new instance of the Image archetype.

        Parameters
        ----------
        data:
             The image data. Should always be a rank-2 or rank-3 tensor.
        draw_order:
             An optional floating point value that specifies the 2D drawing order.
             Objects with higher values are drawn on top of those with lower values.
        """

        # You can define your own __init__ function as a member of ImageExt in image_ext.py
        self.__attrs_init__(data=data, draw_order=draw_order)

    data: components.TensorDataBatch = field(
        metadata={"component": "required"},
        converter=ImageExt.data__field_converter_override,  # type: ignore[misc]
    )
    """
    The image data. Should always be a rank-2 or rank-3 tensor.
    """

    draw_order: components.DrawOrderBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.DrawOrderBatch._optional,  # type: ignore[misc]
    )
    """
    An optional floating point value that specifies the 2D drawing order.
    Objects with higher values are drawn on top of those with lower values.
    """

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__


if hasattr(ImageExt, "deferred_patch_class"):
    ImageExt.deferred_patch_class(Image)
