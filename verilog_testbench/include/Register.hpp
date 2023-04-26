#pragma once

#include <array>
#include <vector>
#include <string>
#include <iostream>

#include "xsi_loader.h"

class Register {
public:

    Register(std::string n, int wdt, Xsi::Loader* ldr) : name(n), width(wdt), loader(ldr) {
        port = loader->get_port_number(name.c_str());
        if(port < 0) {
          std::cerr << "ERROR: Port " << name << " not found" << std::endl;
          exit(1);
        }
    };
    virtual ~Register() = default; 

    auto getWidth() const {return width;};

    friend std::ostream& operator<<(std::ostream& os, const Register& reg);
    Register operator*(const Register& B) const;
    bool operator==(const Register& rhs) const;

protected:
    std::string value = "0";
    unsigned width = 0;
    int port = -1;
    Xsi::Loader* loader;
    std::string name = "Uninitialised";
};

class Input : public Register {
public:
    Input(std::string n, int width, Xsi::Loader* ldr) : Register(n, width, ldr) {};

    void setValue(unsigned val);
    void setValue(std::string val);
    void randomiseValue();
    std::string getValue() const {return value;};
private:
    void sendValue();
};

class Output : public Register {
public:
    Output(std::string n, int width, Xsi::Loader* ldr) : Register(n, width, ldr) {};

    std::string getValue();
private:
    std::string rawOutput;
};

