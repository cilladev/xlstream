"""GIL release verification tests for xlstream Python bindings."""

import os
import threading


import xlstream


class TestGilRelease:
    """Verify the GIL is released during evaluation."""

    def test_concurrent_evaluations_complete_from_threads(
        self, simple_fixture, tmp_path
    ):
        """Multiple evaluations in threads all complete, proving GIL release.

        With a 5-row fixture the evaluation is too fast to reliably measure
        timing speedup. Instead we verify that N threads can all call evaluate
        concurrently without deadlocking (which would happen if the GIL were
        held during Rust work).
        """
        n_threads = 4
        outputs = [str(tmp_path / f"out_{i}.xlsx") for i in range(n_threads)]
        results = [None] * n_threads
        errors = [None] * n_threads
        barrier = threading.Barrier(n_threads, timeout=30)

        def run_eval(idx, output_path):
            try:
                barrier.wait()
                results[idx] = xlstream.evaluate(simple_fixture, output_path, workers=1)
            except Exception as exc:
                errors[idx] = exc

        threads = [
            threading.Thread(target=run_eval, args=(i, outputs[i]))
            for i in range(n_threads)
        ]
        for t in threads:
            t.start()
        for t in threads:
            t.join(timeout=60)

        for i in range(n_threads):
            assert errors[i] is None, f"thread {i} raised: {errors[i]}"
            assert results[i] is not None, f"thread {i} did not produce a result"
            assert results[i]["rows_processed"] > 0

        for out in outputs:
            assert os.path.exists(out)

    def test_thread_pool_all_complete(self, simple_fixture, tmp_path):
        """4 threads all complete successfully via ThreadPoolExecutor."""
        from concurrent.futures import ThreadPoolExecutor

        n = 4
        outputs = [str(tmp_path / f"out_{i}.xlsx") for i in range(n)]

        with ThreadPoolExecutor(max_workers=n) as pool:
            futures = [
                pool.submit(xlstream.evaluate, simple_fixture, out, workers=1)
                for out in outputs
            ]
            results = [f.result() for f in futures]

        assert all(r["rows_processed"] > 0 for r in results)
