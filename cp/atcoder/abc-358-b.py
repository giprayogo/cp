def main():
    _, a = [int(x) for x in input().split()]
    ts = [int(x) for x in input().split()]
    t = 0
    done = []
    for _t in ts:
        if _t < t:
            # queued
            t += a
            done.append(t)
        else:
            # no queue
            t = _t + a
            done.append(t)
    for d in done:
        print(d)


if __name__ == "__main__":
    main()


# * Copy from here up *
import io  # noqa: E402
import sys  # noqa: E402


def test(monkeypatch, capsys):
    # fmt: off
    test_pairs = [
        (
            "3 4\n"
            "0 2 10\n",
            "4\n"
            "8\n"
            "14\n"
        ),
        (
            "3 3\n"
            "1 4 7\n",
            "4\n"
            "7\n"
            "10\n"
        ),
        (
            "10 50000\n"
            "120190 165111 196897 456895 540000 552614 561627 743796 757613 991216\n",
            "170190\n"
            "220190\n"
            "270190\n"
            "506895\n"
            "590000\n"
            "640000\n"
            "690000\n"
            "793796\n"
            "843796\n"
            "1041216\n"
        )
    ]
    # fmt: on

    for i, o in test_pairs:
        mock_input = io.StringIO(i)
        monkeypatch.setattr(sys, "stdin", mock_input)
        main()
        out, _ = capsys.readouterr()
        assert out == o
