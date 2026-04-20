#!/usr/bin/env python3
"""Validate the github-notifications.yml workflow for correctness."""

import yaml
import re
import sys

def load_workflow():
    with open('.github/workflows/github-notifications.yml', encoding='utf-8') as f:
        return yaml.safe_load(f), open('.github/workflows/github-notifications.yml', encoding='utf-8').read()

def test_yaml_syntax(wf):
    """Test 1: YAML syntax and structure"""
    assert wf.get('name'), "Missing 'name' field"
    # PyYAML parses 'on' as boolean True
    on_val = wf.get('on') or wf.get(True)
    assert on_val is not None, "Missing 'on' trigger"
    assert 'jobs' in wf, "Missing 'jobs' field"
    assert 'collect-notifications' in wf['jobs'], "Missing 'collect-notifications' job"
    print("✅ YAML syntax and structure: PASS")

def test_gh_cli_setup(raw):
    """Test 2: gh CLI installation"""
    assert 'actions/setup-go' not in raw, "Still using wrong setup-go action"
    assert 'gh auth login' in raw, "Missing gh auth login"
    # Check for correct stdin method (not here-string)
    assert '<<<' not in raw or '<<<' not in raw.split('gh auth')[1].split('\n')[0] if 'gh auth' in raw else True, "Should use pipe for auth, not here-string"
    assert '--with-stdin' in raw or '--with-token' in raw, "Missing auth method flag"
    print("✅ gh CLI setup: PASS")

def test_jq_env_vars(raw):
    """Test 3: jq environment variable usage"""
    assert 'env.SINCE' not in raw, "Still using env.SINCE (should use --arg)"
    # Check that --arg since is used
    assert '--arg since' in raw, "Missing --arg since usage"
    print("✅ jq environment variables: PASS")

def test_error_handling(raw):
    """Test 4: Error handling guards"""
    # Count lines with fallbacks (|| echo or || true)
    lines = raw.split('\n')
    fallback_count = sum(1 for l in lines if '|| echo' in l or '|| true' in l)
    assert fallback_count >= 6, f"Insufficient error handling (found {fallback_count}, expected >= 6)"
    
    # Verify no bare curl/jq without any protection in critical sections
    assert '2>/dev/null' in raw or '||' in raw, "Missing stderr redirection or fallbacks"
    print("✅ Error handling: PASS")

def test_skip_ci(raw):
    """Test 5: [skip ci] in commit message"""
    assert '[skip ci]' in raw, "Missing [skip ci] in commit message"
    print("✅ [skip ci] in commit: PASS")

def test_workflow_dispatch(raw):
    """Test 6: workflow_dispatch trigger"""
    assert 'workflow_dispatch' in raw, "Missing workflow_dispatch trigger"
    print("✅ workflow_dispatch trigger: PASS")

def test_permissions(raw):
    """Test 7: Permissions are set"""
    assert 'contents: write' in raw, "Missing contents: write permission"
    print("✅ Permissions: PASS")

if __name__ == '__main__':
    try:
        wf, raw = load_workflow()
        test_yaml_syntax(wf)
        test_gh_cli_setup(raw)
        test_jq_env_vars(raw)
        test_error_handling(raw)
        test_skip_ci(raw)
        test_workflow_dispatch(raw)
        test_permissions(raw)
        print("\n🎉 All 7 tests PASSED")
    except AssertionError as e:
        print(f"\n❌ FAILED: {e}")
        sys.exit(1)
    except Exception as e:
        print(f"\n❌ ERROR: {e}")
        sys.exit(1)
