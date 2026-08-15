"""Regression tests for the source-controlled v2 connectivity utility."""

import os
import unittest
from unittest.mock import patch

import verify_v2_connectivity as connectivity


class ConnectivityConfigurationTests(unittest.TestCase):
    def test_configured_nodes_require_runtime_tokens(self) -> None:
        with patch.dict(os.environ, {}, clear=True):
            with self.assertRaisesRegex(RuntimeError, "GHOSTWIRE_NODE_ALPHA_TOKEN"):
                connectivity.configured_nodes()

    def test_configured_nodes_read_tokens_from_environment(self) -> None:
        environment = {
            "GHOSTWIRE_NODE_ALPHA_TOKEN": "test-alpha-token",
            "GHOSTWIRE_NODE_BETA_TOKEN": "test-beta-token",
        }
        with patch.dict(os.environ, environment, clear=True):
            nodes = connectivity.configured_nodes()

        self.assertEqual(nodes["node_alpha"]["url"], "http://localhost:3001")
        self.assertEqual(nodes["node_beta"]["url"], "http://localhost:3002")
        self.assertEqual(nodes["node_alpha"]["token"], "test-alpha-token")
        self.assertEqual(nodes["node_beta"]["token"], "test-beta-token")
if __name__ == "__main__":
    unittest.main()
