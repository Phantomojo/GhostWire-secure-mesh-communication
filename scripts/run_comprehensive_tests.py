#!/usr/bin/env python3
"""
🧪 Comprehensive Testing Suite for GhostWire
Runs all tests, security checks, and generates comprehensive reports
"""

import os
import sys
import json
import subprocess
import time
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Any, Optional

class GhostWireTestSuite:
    """Comprehensive testing suite for GhostWire project"""
    
    def __init__(self):
        self.project_root = Path.cwd()
        self.results_dir = self.project_root / "test_results"
        self.logs_dir = self.project_root / "logs"
        self.reports_dir = self.project_root / "reports"
        
        # Create directories
        self.results_dir.mkdir(exist_ok=True)
        self.logs_dir.mkdir(exist_ok=True)
        self.reports_dir.mkdir(exist_ok=True)
        
        self.timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        self.test_results = {}
        
    def log(self, message: str, level: str = "INFO"):
        """Log a message with timestamp"""
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        log_message = f"[{timestamp}] [{level}] {message}"
        print(log_message)
        
        # Also write to log file
        log_file = self.logs_dir / f"test-suite-{self.timestamp}.log"
        with open(log_file, "a") as f:
            f.write(log_message + "\n")
    
    def run_command(self, command: List[str], description: str) -> Dict[str, Any]:
        """Run a command and return results"""
        self.log(f"Running: {description}")
        self.log(f"Command: {' '.join(command)}")
        
        start_time = time.time()
        result = {
            "command": command,
            "description": description,
            "start_time": start_time,
            "success": False,
            "output": "",
            "error": "",
            "exit_code": None,
            "duration": 0
        }
        
        try:
            process = subprocess.run(
                command,
                capture_output=True,
                text=True,
                cwd=self.project_root,
                timeout=300  # 5 minute timeout
            )
            
            result["exit_code"] = process.returncode
            result["output"] = process.stdout
            result["error"] = process.stderr
            result["success"] = process.returncode == 0
            result["duration"] = time.time() - start_time
            
            if result["success"]:
                self.log(f"✅ {description} completed successfully in {result['duration']:.2f}s")
            else:
                self.log(f"❌ {description} failed with exit code {process.returncode}")
                
        except subprocess.TimeoutExpired:
            result["error"] = "Command timed out after 5 minutes"
            result["success"] = False
            self.log(f"⏰ {description} timed out")
        except Exception as e:
            result["error"] = str(e)
            result["success"] = False
            self.log(f"💥 {description} failed with exception: {e}")
        
        return result
    
    def check_dependencies(self) -> Dict[str, Any]:
        """Check if required dependencies are installed"""
        self.log("🔍 Checking dependencies...")
        
        dependencies = {
            "pytest": "pytest --version",
            "black": "black --version",
            "flake8": "flake8 --version",
            "mypy": "mypy --version",
            "bandit": "bandit --version",
            "safety": "safety --version"
        }
        
        results = {}
        missing = []
        
        for dep, command in dependencies.items():
            try:
                result = subprocess.run(
                    command.split(),
                    capture_output=True,
                    text=True,
                    timeout=10
                )
                if result.returncode == 0:
                    results[dep] = {
                        "installed": True,
                        "version": result.stdout.strip()
                    }
                else:
                    results[dep] = {"installed": False, "version": None}
                    missing.append(dep)
            except Exception:
                results[dep] = {"installed": False, "version": None}
                missing.append(dep)
        
        if missing:
            self.log(f"⚠️ Missing dependencies: {', '.join(missing)}", "WARNING")
        else:
            self.log("✅ All dependencies are installed")
            
        return {"dependencies": results, "missing": missing}
    
    def run_unit_tests(self) -> Dict[str, Any]:
        """Run unit tests with pytest"""
        self.log("🧪 Running unit tests...")
        
        # Check if tests directory exists
        tests_dir = self.project_root / "tests"
        if not tests_dir.exists():
            self.log("⚠️ No tests directory found, skipping unit tests", "WARNING")
            return {"success": False, "reason": "No tests directory found"}
        
        command = [
            "python", "-m", "pytest",
            "tests/",
            "-v",
            "--cov=ghostwire",
            "--cov-report=xml",
            "--cov-report=html",
            "--cov-report=term-missing",
            "--junitxml=test-results.xml"
        ]
        
        result = self.run_command(command, "Unit Tests")
        
        # Parse test results if available
        if result["success"] and "test-results.xml" in result["output"]:
            result["test_results"] = self.parse_test_results()
        
        return result
    
    def run_code_quality_checks(self) -> Dict[str, Any]:
        """Run code quality checks"""
        self.log("🔍 Running code quality checks...")
        
        results = {}
        
        # Black formatting check
        black_result = self.run_command(
            ["black", "--check", "--diff", "."],
            "Black Code Formatting Check"
        )
        results["black"] = black_result
        
        # isort import sorting check
        isort_result = self.run_command(
            ["isort", "--check-only", "--diff", "."],
            "isort Import Sorting Check"
        )
        results["isort"] = isort_result
        
        # flake8 linting
        flake8_result = self.run_command(
            ["flake8", ".", "--count", "--select=E9,F63,F7,F82", "--show-source", "--statistics"],
            "Flake8 Linting (Errors)"
        )
        results["flake8_errors"] = flake8_result
        
        # flake8 style check
        flake8_style_result = self.run_command(
            ["flake8", ".", "--count", "--exit-zero", "--max-complexity=10", "--max-line-length=88", "--statistics"],
            "Flake8 Style Check"
        )
        results["flake8_style"] = flake8_style_result
        
        # mypy type checking
        mypy_result = self.run_command(
            ["mypy", "ghostwire/", "--ignore-missing-imports"],
            "MyPy Type Checking"
        )
        results["mypy"] = mypy_result
        
        return results
    
    def run_security_checks(self) -> Dict[str, Any]:
        """Run security checks"""
        self.log("🔒 Running security checks...")
        
        results = {}
        
        # Bandit security scan
        bandit_result = self.run_command(
            ["bandit", "-r", "ghostwire/", "-f", "json", "-o", "bandit-results.json"],
            "Bandit Security Scan"
        )
        results["bandit"] = bandit_result
        
        # Safety vulnerability check
        safety_result = self.run_command(
            ["safety", "check", "--json", "--output", "safety-results.json"],
            "Safety Vulnerability Check"
        )
        results["safety"] = safety_result
        
        return results
    
    def run_integration_tests(self) -> Dict[str, Any]:
        """Run integration tests"""
        self.log("🔗 Running integration tests...")
        
        # Check if docker-compose exists
        if not (self.project_root / "docker-compose.yml").exists():
            self.log("⚠️ No docker-compose.yml found, skipping integration tests", "WARNING")
            return {"success": False, "reason": "No docker-compose.yml found"}
        
        # Start services
        self.log("🚀 Starting services for integration tests...")
        start_result = self.run_command(
            ["docker-compose", "up", "-d"],
            "Start Docker Services"
        )
        
        if not start_result["success"]:
            return {"success": False, "reason": "Failed to start services"}
        
        # Wait for services to be ready
        time.sleep(10)
        
        # Run integration tests
        integration_result = self.run_command(
            ["python", "-m", "pytest", "tests/", "-m", "integration", "-v"],
            "Integration Tests"
        )
        
        # Stop services
        self.run_command(
            ["docker-compose", "down"],
            "Stop Docker Services"
        )
        
        return integration_result
    
    def parse_test_results(self) -> Dict[str, Any]:
        """Parse test results from pytest output"""
        # This is a simplified parser - you might want to use a proper XML parser
        return {"parsed": False, "note": "Test results parsing not implemented yet"}
    
    def generate_report(self) -> str:
        """Generate comprehensive test report"""
        self.log("📊 Generating comprehensive test report...")
        
        report_file = self.reports_dir / f"comprehensive-test-report-{self.timestamp}.md"
        
        with open(report_file, "w") as f:
            f.write(f"""# 🧪 Comprehensive Test Report - GhostWire
Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}
Timestamp: {self.timestamp}

## 📋 Test Summary

### Dependencies
""")
            
            # Add dependency results
            if "dependencies" in self.test_results:
                deps = self.test_results["dependencies"]
                for dep, info in deps["dependencies"].items():
                    status = "✅" if info["installed"] else "❌"
                    version = info["version"] or "Not installed"
                    f.write(f"- {status} **{dep}**: {version}\n")
                
                if deps["missing"]:
                    f.write(f"\n⚠️ **Missing Dependencies**: {', '.join(deps['missing'])}\n")
            
            f.write(f"""
### Test Results
""")
            
            # Add test results
            if "unit_tests" in self.test_results:
                ut = self.test_results["unit_tests"]
                status = "✅" if ut["success"] else "❌"
                f.write(f"- {status} **Unit Tests**: {ut['description']}\n")
                if ut["success"]:
                    f.write(f"  - Duration: {ut['duration']:.2f}s\n")
                else:
                    f.write(f"  - Error: {ut['error']}\n")
            
            if "code_quality" in self.test_results:
                cq = self.test_results["code_quality"]
                f.write(f"\n- 🔍 **Code Quality Checks**:\n")
                for tool, result in cq.items():
                    status = "✅" if result["success"] else "❌"
                    f.write(f"  - {status} {result['description']}\n")
            
            if "security_checks" in self.test_results:
                sc = self.test_results["security_checks"]
                f.write(f"\n- 🔒 **Security Checks**:\n")
                for tool, result in sc.items():
                    status = "✅" if result["success"] else "❌"
                    f.write(f"  - {status} {result['description']}\n")
            
            if "integration_tests" in self.test_results:
                it = self.test_results["integration_tests"]
                status = "✅" if it["success"] else "❌"
                f.write(f"\n- 🔗 **Integration Tests**: {status}\n")
                if not it["success"]:
                    f.write(f"  - Reason: {it['reason']}\n")
            
            f.write(f"""
## 📁 Generated Files
- Test Results: `{self.results_dir}/`
- Logs: `{self.logs_dir}/`
- Reports: `{self.reports_dir}/`

## 🎯 Recommendations
1. Address any failed tests immediately
2. Fix code quality issues
3. Resolve security vulnerabilities
4. Ensure all dependencies are properly installed
5. Review integration test failures

## 📊 Overall Status
""")
            
            # Calculate overall status
            total_tests = 0
            passed_tests = 0
            
            for test_type, result in self.test_results.items():
                if isinstance(result, dict) and "success" in result:
                    total_tests += 1
                    if result["success"]:
                        passed_tests += 1
            
            if total_tests > 0:
                success_rate = (passed_tests / total_tests) * 100
                f.write(f"- **Success Rate**: {success_rate:.1f}% ({passed_tests}/{total_tests})\n")
                
                if success_rate == 100:
                    f.write("- **Status**: 🎉 All tests passed!\n")
                elif success_rate >= 80:
                    f.write("- **Status**: ⚠️ Most tests passed, some issues to address\n")
                else:
                    f.write("- **Status**: 🚨 Significant test failures detected\n")
            else:
                f.write("- **Status**: No tests were run\n")
        
        self.log(f"📊 Report generated: {report_file}")
        return str(report_file)
    
    def run_all_tests(self) -> Dict[str, Any]:
        """Run all tests and checks"""
        self.log("🚀 Starting comprehensive test suite...")
        
        start_time = time.time()
        
        # Check dependencies
        self.test_results["dependencies"] = self.check_dependencies()
        
        # Run unit tests
        self.test_results["unit_tests"] = self.run_unit_tests()
        
        # Run code quality checks
        self.test_results["code_quality"] = self.run_code_quality_checks()
        
        # Run security checks
        self.test_results["security_checks"] = self.run_security_checks()
        
        # Run integration tests
        self.test_results["integration_tests"] = self.run_integration_tests()
        
        # Generate report
        report_file = self.generate_report()
        
        total_duration = time.time() - start_time
        
        # Summary
        self.log("🎉 Comprehensive test suite completed!")
        self.log(f"⏱️ Total duration: {total_duration:.2f}s")
        self.log(f"📊 Report generated: {report_file}")
        
        return {
            "success": True,
            "duration": total_duration,
            "report_file": report_file,
            "results": self.test_results
        }

def main():
    """Main entry point"""
    print("🧪 GhostWire Comprehensive Test Suite")
    print("=" * 50)
    
    test_suite = GhostWireTestSuite()
    
    try:
        results = test_suite.run_all_tests()
        
        if results["success"]:
            print(f"\n✅ Test suite completed successfully in {results['duration']:.2f}s")
            print(f"📊 Check the report: {results['report_file']}")
            sys.exit(0)
        else:
            print("\n❌ Test suite failed")
            sys.exit(1)
            
    except KeyboardInterrupt:
        print("\n⏹️ Test suite interrupted by user")
        sys.exit(130)
    except Exception as e:
        print(f"\n💥 Test suite failed with error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
