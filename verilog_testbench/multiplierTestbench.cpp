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

   // my variables
   int status = 0;

   try {
      Xsi::Loader XSI(design_libname, simengine_libname);
      s_xsi_setup_info info;
      memset(&info, 0, sizeof(info));
      info.logFileName = NULL;
      char wdbName[] = "test.wdb";
      info.wdbFileName = wdbName;
      XSI.open(&info);
      XSI.trace_all();

      Input IN1("IN1", 22, &XSI);
      Input IN2("IN2", 34, &XSI);
      Output OUTPUT("OUTPUT", 56, &XSI);

      // Start low clock
      XSI.run(10);

      // The reset is done. Now start counting
      std::cout << "\n *** START TEST ***\n";

      // std::cout << "At testcase: " << testcase.IN1 << std::endl;
      IN2.setValue("101010101100110011");
      IN1.setValue("1000101010101010101");
      auto expected = multiply(IN1, IN2);
      XSI.run(10);
      auto res = OUTPUT.getValue();

      std::cout << IN1 << IN2;
      std::cout << "Expected: " << 2 << "\n";
      std::cout << "C++ Expected: " << expected << '\n';
      std::cout << OUTPUT;
      std::cout << "Raw outut: " << res << '\n';

      for(int i = 0; i < 5; i++) {
         IN1.randomiseValue();
         IN2.randomiseValue();
         auto expected = multiply(IN1, IN2);
         XSI.run(1000);
         auto res = OUTPUT.getValue();
         if(res != expected) {
            std::cout << "TEST FAILED \n";
            std::cout << IN1 << IN2;
            std::cout << OUTPUT;
            std::cout << "C++ Expected: " << expected << '\n';
         }
      }


      std::cout << "\n *** END TEST ***\n";
      // Just a check to rewind time to 0
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

   if (status == 0) {
      std::cout << "PASSED test\n";
   }
   else {
      std::cout << "FAILED test\n";
   }

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