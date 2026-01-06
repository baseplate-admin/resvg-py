import resvg_py
import base64

svg_string = """
    <svg width="300" height="130" xmlns="http://www.w3.org/2000/svg">
      <rect width="200" height="100" x="10" y="10" rx="20" ry="20" fill="blue" />
    </svg>
"""


def test_rectangle():
    _base64 = base64.b64encode(
        bytes(resvg_py.svg_to_bytes(svg_string=svg_string))
    ).decode("utf-8")
    assert (
        _base64
        == "iVBORw0KGgoAAAANSUhEUgAAASwAAACCCAYAAAD8HPVfAAAHn0lEQVR4Ae3AA6AkWZbG8f937o3IzKdyS2Oubdu2bdu2bdu2bWmMnpZKr54yMyLu+Xa3anqmhztr1U9cddVVV/3vgLjqqquu+t8BcdVVV131vwPiqquuuup/B8RVV1111f8OiKuuuuqq/x0QV1111VX/OyCuuuqqq/53QFx11VVX/e+AuOqqq6763wHxn84vDbwV8NLAg4GX5qqr/uf4a+BW4K+BnwH9NVf9T4X4T+EHAx8FvDXwYK666n+PW4GfBr4GdCtX/U+C+A/l48BXAe/NVVf97/fdwMeAdrnqfwLEfxi/NfBdwHGuuur/jl3gfUA/zVX/3RD/IfxdwHtz1VX/d3036H246r8T4t/Fx4HvAt6aq676v++ngfcB7XLVfwfEv4t/Cnhrrrrq/4+fBr0NV/13QPyb+buA9+aqq/7/+RrQR3PVfzXEv4nfGvgprrrq/6+3Af00V/1XQvyr+TjwdOA4V131/9cu8BDQLlf9V0H8q/m7gffiqquu+h7Qe3PVfxXEv4ofDDydq6666n4PAd3KVf8VEP8q/mrgo7jqqqvu9zWgj+aq/wqIfxU/HXgwV1111f1uBT2Eq/4rIF5kfmngr7jqqque28uA/pqr/rMhXmT+bOCzuOqqq57bx4C+mqv+syFeZP5p4K246qqrntvPgN6aq/6zIV5k/ivgpbnqqque21+DXoar/rMhXmQ2V1111QsgcdV/NsSLzOaqq656ASSu+s+GeJHZXHXVVS+AxFX/2RAvMpurrrrqBZC46j8b4kVmc9VVV70AElf9Z0O8yGyuuuqqF0Diqv9siBeZzVVXXfUCSFz1nw3xIrO56qqrXgCJq/6zIV5kNlddddULIHHVfzbEi8zmqquuegEkrvrPhniR2Vx11VUvgMRV/9kQLzKbq6666gWQuOo/G+JFZnPVVVe9ABJX/WdDvMhsrrrqqhdA4qr/bIgXmc1VV131Akhc9Z8N8SKzueqqq14Aiav+syFeZDZXXXXVCyBx1X82xIvM5qqrrnoBJK76z4Z4kdlcddVVL4DEVf/ZEC8ym6uuuuoFkLjqPxviRWZz1VVXvQASV/1nQ7zIbK666qoXQOKq/2yIF5nNVVdd9QJIXPWfDfEis7nqqqteAImr/rMhXmQ2V1111QsgcdV/NsSLzOaqq656ASSu+s+GeJHZXHXVVS+AxFX/2RAvMpurrrrqBZC46j8b4kVmc9VVV70AElf9Z0O8yGyuuuqqF0Diqv9siBeZzVVXXfUCSFz1nw3xIrO56qqrXgCJq/6zIV5kNlddddULIHHVfzbEi8zmqquuegEkrvrPhniR2Vx11VUvgMRV/9kQLzKbq6666gWQuOo/G+JFZnPVVVe9ABJX/WdDvMhsrrrqqhdA4qr/bIgXmc1VV131Akhc9Z8N8SKzueqqq14Aiav+syFeZDZXXXXVCyBx1X82xIvM5qqrrnoBJK76z4Z4kdlcddVVL4DEVf/ZEC8ym6uuuuoFkLjqPxviRWZz1VVXvQASV/1nQ7zIbK666qoXQOKq/2yIF5nNVVdd9QJIXPWfDfEis7nqqqteAImr/rMhXmQ2V1111QsgcdV/NsSLzOaqq656ASSu+s+GeJHZXHXVVS+AxFX/2RAvMpurrrrqBZC46j8b4kVmc9VVV70AElf9Z0O8yGyuuuqqF0Diqv9siBeZzVVXXfUCSFz1nw3xIrO56qqrXgCJq/6zIV5kNlddddULIHHVfzbEi8zmqquuegEkrvrPhniR2Vx11VUvgMRV/9kQLzKbq6666gWQuOo/G+JFZnPVVVe9ABJX/WdDvMhsrrrqqhdA4qr/bIgXmc1VV131Akhc9Z8N8SKzueqqq14Aiav+syFeZP5r4KW46qqrntvfgF6aq/6zIV5k/mngrbjqqque28+A3pqr/rMhXmT+bOCzuOqqq57bx4C+mqv+syFeZH5p4K+46qqrntvLgP6aq/6zIf5VfCvwIK666qr7PQP0YK76r4D4V/FXAx/FVVdddb+vAX00V/1XQPyr+MHA07nqqqvu9xDQrVz1XwHxr+bvBt6Lq6666mtAH81V/1UQ/2o+DtwKHOOqq/7/ugQ8GLTLVf9VEP8mfmvgp7jqqv+/3gb001z1Xwnxb+bvBt6Lq676/+drQB/NVf/VEP8u/mngrbjqqv8/fgb01lz13wHx7+LjwHcDb8VVV/3f9zPAe4N2ueq/A+I/hL8beC+uuur/rq8BfTRX/XdC/IfxWwPfDRzjqqv+77gEvDfop7nqvxviP5SPA18NvBdXXfW/39cAnw3a5ar/CRD/Kfxg4KOBtwYexFVX/e/xDOCnga8G3cpV/5Mg/tP5pYHXBl4beDDwUlx11f8cfwPcCvw28Nugv+aq/6kQV1111VX/OyCuuuqqq/53QFx11VVX/e+AuOqqq6763wFx1VVXXfW/A+Kqq6666n8HxFVXXXXV/w6Iq6666qr/HRBXXXXVVf87IK666qqr/ndAXHXVVVf974C46qqrrvrfAXHVVVdd9b8D4qqrrrrqfwfEVVddddX/Doirrrrqqv8dEFddddVV/zsgrrrqqqv+d0BcddVVV/3vgLjqqquu+t+BfwQkpP6DQU7d0QAAAABJRU5ErkJggg=="
    )
