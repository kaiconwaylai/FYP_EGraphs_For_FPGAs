#include <stdlib.h>
#include <string>
#include <cstring>
#include <iostream>

#include "./include/Register.hpp"
#include "./include/testValues.hpp"
#include "./include/xsi_loader.h"
#include "./include/multiplier.hpp"

const char *expected_out[15] = {"0001", "0010", "0011", "0100", "0101", "0110", "0111", "1000", "1001", "1010", "1011", "1100", "1101", "1110", "1111"};

std::string getcurrentdir();

int main(int argc, char **argv) {

   const unsigned IN1_WIDTH = 32;
   const unsigned IN2_WIDTH = 32;
   const unsigned OUTPUT_WIDTH = 64;

   auto cwd = getcurrentdir();
   std::string simengine_libname = "librdi_simulator_kernel";

#if defined(_WIN32)
   const char *lib_extension = ".dll";
#else
   const char *lib_extension = ".so";
#endif
   simengine_libname += lib_extension;

   // std::string design_libname = getcurrentdir() + "/xsim.dir/counter/xsimk" + lib_extension;
   std::string design_libname = "xsim.dir/mult/xsimk";
   design_libname = design_libname + lib_extension;

   std::cout << "Design DLL     : " << design_libname << std::endl;
   std::cout << "Sim Engine DLL : " << simengine_libname << std::endl;

   int status = 0;
   auto testCases = standardiseUnitTests(OUTPUT_WIDTH);
   unsigned testsCompleted = 0;
   unsigned testsPassed = 0;

   try {
      Xsi::Loader XSI(design_libname, simengine_libname);
      s_xsi_setup_info info;
      memset(&info, 0, sizeof(info));
      info.logFileName = NULL;
      char wdbName[] = "test.wdb";
      info.wdbFileName = wdbName;
      XSI.open(&info);
      XSI.trace_all();

      Input IN1("IN1", IN1_WIDTH, &XSI);
      Input IN2("IN2", IN2_WIDTH, &XSI);
      Output OUTPUT("OUTPUT", OUTPUT_WIDTH, &XSI);

      std::cout << "\n *** START TESTING ***\n";

      for(const auto& testcase : testCases) {
         IN1.setValue(testcase.IN1);
         IN2.setValue(testcase.IN2);
         XSI.run(1);
         auto res = OUTPUT.getValue();
         if(res != testcase.EXPECTED) {
            std::cout << "TEST FAILED \n";
            std::cout << IN1 << IN2 << OUTPUT;
            std::cout << "Expected: " << testcase.EXPECTED << '\n';
            testsPassed--;
         }
         testsCompleted++; testsPassed++;
      }

      for(int i = 0; i < 10; i++) {
         IN1.randomiseValue();
         IN2.randomiseValue();
         auto expected = multiply(IN1, IN2);
         XSI.run(1);
         auto res = OUTPUT.getValue();
         if(res != expected) {
            std::cout << "TEST FAILED \n";
            std::cout << IN1 << IN2 << OUTPUT;
            std::cout << "Expected: " << expected << '\n';
            testsPassed--;
         }
         testsCompleted++; testsPassed++;
      }

      std::cout << "\n *** END TESTING ***\n";

      XSI.restart();
   }
   catch (std::exception &e) {
      std::cerr << "ERROR: An exception occurred: " << e.what() << std::endl;
      status = 2;
   }
   catch (const char* msg) {
      std::cerr << "ERROR: An exception occured:" << msg << std::endl;
      status = 3;
   }
   catch (...) {
      std::cerr << "ERROR: An unknown exception occurred." << std::endl;
      status = 4;
   }

   std::cout << "Passed " << testsPassed << "/" << testsCompleted << " tests. \n";

   exit(status);
}

std::string getcurrentdir() {
#if defined(_WIN32)
   char buf[MAX_PATH];
   GetCurrentDirectory(sizeof(buf), buf);
   buf[sizeof(buf) - 1] = 0;
   return buf;
#else
   char buf[1024];
   // getcwd(buf, sizeof(buf)-1);
   buf[sizeof(buf) - 1] = 0;
   return buf;
#endif
}