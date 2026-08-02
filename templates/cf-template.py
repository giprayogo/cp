"""Python template for CodeForces/AtCoder style competition."""


def main():
    i = input()
    if i == "input":
        print("output")


if __name__ == "__main__":
    main()

# * Copy from here up *
import io  # noqa: E402
import sys  # noqa: E402


def test(monkeypatch, capsys):
    # fmt: off
    io_pairs = [
        (
            "input\n",
            "output\n"
        ),
    ]
    # fmt: on

    for i, o in io_pairs:
        mock_input = io.StringIO(i)
        monkeypatch.setattr(sys, "stdin", mock_input)
        main()
        out, _ = capsys.readouterr()
        assert out == o
