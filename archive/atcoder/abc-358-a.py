def main():
    a, b = input().split()
    if a == "AtCoder" and b == "Land":
        print("Yes")
    else:
        print("No")


if __name__ == "__main__":
    main()


# * Copy from here up *
import io  # noqa: E402
import sys  # noqa: E402


def test(monkeypatch, capsys):
    # fmt: off
    test_pairs = [
        (
            "AtCoder Land\n",
            "Yes\n"
        ),
        (
            "aTcodeR lANd\n",
            "No\n"
        ),
    ]
    # fmt: on

    for i, o in test_pairs:
        mock_input = io.StringIO(i)
        monkeypatch.setattr(sys, "stdin", mock_input)
        main()
        out, _ = capsys.readouterr()
        assert out == o
