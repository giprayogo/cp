"""Python template for CodeForces/AtCoder style competition."""


def main():
    n = int(input())
    for _ in range(n):
        text = input().strip()
        print(len(text.split()))


if __name__ == "__main__":
    main()

# * Copy from here up *
import io  # noqa: E402
import sys  # noqa: E402


def test(monkeypatch, capsys):
    # fmt: off
    mock_input = io.StringIO(
        "3\n"
        "a quick brown fox jumps\n"
        "lorem ipsum\n"
        "input line 3 !\n"
    )
    # fmt: on
    monkeypatch.setattr(sys, "stdin", mock_input)

    main()

    out, _ = capsys.readouterr()
    # fmt: off
    assert out == (
        "5\n"
        "2\n"
        "4\n"
    )
    # fmt: on
