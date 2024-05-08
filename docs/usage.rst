Usage
=====


The module takes in **utf-8** encoded `svg_string` and returns `base64` encoded **PNG** string.


Lets say our svg looks like this :

.. raw:: html

    <center> 
        <svg width="300" height="130" xmlns="http://www.w3.org/2000/svg">
            <rect width="200" height="100" x="10" y="10" rx="20" ry="20" fill="blue" />
        </svg>
    </center>


We can convert it to `PNG` by:


.. code-block:: python

    
    import resvg_py

    svg_string = """
        <svg width="300" height="130" xmlns="http://www.w3.org/2000/svg">
          <rect width="200" height="100" x="10" y="10" rx="20" ry="20" fill="blue" />
        </svg>
    """

    png_bytes: list[bytes] = resvg_py.svg_to_bytes(svg_string=svg_string) # a large list of bytes 



In order to convert image to base64:

.. code-block:: python

    import resvg_py
    import base64

    svg_string = """
        <svg width="300" height="130" xmlns="http://www.w3.org/2000/svg">
          <rect width="200" height="100" x="10" y="10" rx="20" ry="20" fill="blue" />
        </svg>
    """

    # a large list of bytes
    png_bytes: list[bytes] = resvg_py.svg_to_bytes(svg_string=svg_string)
    base64_utf8_str = base64.b64encode(bytes(png_bytes)).decode("utf-8")
    print(f"data:image/png;base64,{base64_utf8_str}")


This should return the following **PNG** image (check using inspect element):

.. raw:: html

    <center> 
        <img src="data:image/png;base64,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" ></img>
    </center>


We can also do something like this :

.. code-block:: python

    
    import resvg_py

    svg = ... # path to svg file

    print(resvg_py.svg_to_bytes(svg_path=svg)) 



But please do note that `resvg` first looks for *svg_string* and if that is empty then it looks for *svg_path*


For extra parameters refer to :doc:`resvg <../resvg>` 