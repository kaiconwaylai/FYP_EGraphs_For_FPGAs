#include "../include/Register.hpp"
#include <stdlib.h>     /* srand, rand */
#include <algorithm>
#include <cmath>
#include<ctime>


void Input::setValue(unsigned val) {
    if(val = 0) {
        value = "0";
        return;
    }
    if(val >= std::pow(2,width)) {
        throw "Attempt to assign too large a value to input";
    }

    std::string tmp = "";
    while(val > 0) {
        if(val & 1) {
            tmp.push_back('1');
        } else {
            tmp.push_back('0');
        }
    }

    sendValue();
}

void Input::setValue(const std::string& val) {
    if(val.length() - val.find("1") > width) {
        throw "Attempt to assign too large a value to input";
    }

    for(const char c : val) {
        if(c != '0' && c != '1') {
            throw "Invalid input number";
        }
    }
    value = std::string(width-val.length(), '0') + val;
    sendValue();
}

void Input::randomiseValue() {
    srand(time(0));
    std::string newVal(width, 0);
    for(char& i : newVal) {
        if(rand()%2) {
            i = '1';
        } else {
            i = '0';
        }
    }
    value = newVal;
    sendValue();
}


void Input::sendValue() {
    unsigned arraySize = std::ceil(width/32.0);
    std::vector<s_xsi_vlog_logicval> data(arraySize, {0,0});

    for(unsigned i = 0; i < arraySize; i++) {
        int offset = width - (1 + i)*32;
        int start = offset > 0 ? offset : 0;
        int length = offset >= 0 ? 32 : width % 32;
        auto cur = value.substr(start,length);
        data[i] = {std::stoul(cur,0,2),0};
    }
    
	loader->put_value(port, &data[0]);
    loader->run(10);

}


void append_logic_val_bit_to_string(std::string& retVal, int aVal, int bVal)
{
     if(aVal == 0) {
        if(bVal == 0) {
           retVal +="0";
        } else {
           retVal +="Z";
        }
     } else { // aVal == 1
        if(bVal == 0) {
           retVal +="1";
        } else {
           retVal +="X";
        }
     }
}


void append_logic_val_to_string(std::string& retVal, int aVal, int bVal, int max_bits)
{
   int bit_mask = 0X00000001;
   int aVal_bit, bVal_bit;
   for(int k=max_bits; k>=0; k--) {
      aVal_bit = (aVal >> k ) & bit_mask;
      bVal_bit = (bVal >> k ) & bit_mask;
      append_logic_val_bit_to_string(retVal, aVal_bit, bVal_bit);
   }
}

std::string logic_val_to_string(s_xsi_vlog_logicval* value, int size)
{
   std::string retVal;

   int num_words = size/32 + 1;
   int max_lastword_bit = size %32 - 1;

   // last word may have unfilled bits
   int  aVal = value[num_words -1].aVal;
   int  bVal = value[num_words -1].bVal;
   append_logic_val_to_string(retVal, aVal, bVal, max_lastword_bit);
   
   // this is for fully filled 32 bit aVal/bVal structs
   for(int k = num_words - 2; k>=0; k--) {
      aVal = value[k].aVal;
      bVal = value[k].bVal;
      append_logic_val_to_string(retVal, aVal, bVal, 31);
   }
   return retVal;
}

std::string Output::getValue() {
    unsigned arraySize = std::ceil(width/32.0);
    std::vector<s_xsi_vlog_logicval> outputVal(arraySize,{0,0});

    loader->get_value(port, &outputVal[0]);

    for(const auto& sanity : outputVal) {
        if(sanity.bVal != 0) 
            throw "Simulation output contains unspecified values";
    }

    /* Transform into a string we can sanely deal with */
    std::string retVal = "";
    for(int i = arraySize - 1; i > -1; i--) {
        retVal += logic_val_to_string(&outputVal[i], 32);
    }
    value = retVal.substr(retVal.length() - width);
    return value;
}

bool Register::operator==(const Register& rhs) const {
    return value == rhs.value;
}

std::ostream& operator<<(std::ostream& os, const Register& reg) {
    os << reg.name << ": " << reg.value << "\n";
    return os;
}