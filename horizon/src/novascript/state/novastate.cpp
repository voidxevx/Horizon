#include "novastate.h"

#include <stdio.h>
#include <fstream>
#include <cassert>
#include <sstream>

#include <rust/cxx.h>

namespace nova
{

    State::State()
    {
    }

    std::shared_ptr<State>
    new_novastate()
    {
        return std::make_shared<State>(State{});
    }

    void
    State::linkModule(const std::string& moduleName, const std::string& filePath)
    {
        propID id = s_PropertyHasher(moduleName);
        if (m_Modules.count(id) == 0)
            m_Modules[id] = filePath;
    }

    void
    State::loadModule(const std::string& moduleName)
    {
        propID id = s_PropertyHasher(moduleName);
        loadModule(id);
    }

    void
    State::loadModule(propID moduleID)
    {
        if (m_Modules.count(moduleID) > 0)
        {
            if (std::holds_alternative<std::string>(m_Modules.at(moduleID)))
            {
                const std::string path = std::get<std::string>(m_Modules.at(moduleID));
                const gen::TokenPackage package = tokenizeFile(path);
                // build tokens
                // store module package
            }
        }
        else
            printf("\033[33m** [NOVA][CPP] Unable to load module, module id not found.\033[0m\n");
    }











    ///////////////////////
    // FILE TOKENIZATION //
    ///////////////////////

    const gen::TokenPackage
    State::tokenizeFile(const std::string& filePath)
    {
        gen::TokenPackage package{};

        std::ifstream file{};
        file.open(filePath);
        assert(file.is_open() && "*** [NOVA][CPP] Unable to open nova source file!");

        std::string buffer{};

        // iteratr up to the dependencies list.
        while (std::getline(file, buffer) && buffer != "_dep:");

        // load dependant modules
        while (std::getline(file, buffer) && buffer != "_code:")
        if (buffer.size() > 1)
            loadModule(buffer);

        bool exclude = false;
        while (std::getline(file, buffer))
        {
            size_t index = 0;
            std::stringstream tBuffer{};

            bool skipLine = false;
            while (index < buffer.size() && !skipLine)
            {
                char c_char{ buffer[index] };

                if (exclude)
                {
                    if (c_char == '*' && buffer[index + 1] == '/')
                    {
                        exclude = false;
                        ++index;
                    }
                    ++index;
                }


                else if (isspace(c_char))
                    ++index;

                else if (isdigit(c_char))
                {
                    // TODO: immediate numbers
                    ++index;
                }

                // keyword & identifiers
                else if (isalnum(c_char) || c_char == '_')
                {
                    bool allow_symbols = c_char == '_';
                    while((isalnum(c_char) || (allow_symbols && !isspace(c_char))) && index < buffer.size())
                    {
                        tBuffer << c_char;
                        ++index;
                        c_char = buffer[index];
                    }

                    const std::string str{ tBuffer.str() };
                    tBuffer.str("");

                    if (gen::s_TokenMap.count(str) > 0)
                        package.Tokens.push_back(gen::Token(gen::s_TokenMap[str]));
                    else
                    {
                        propID id = s_PropertyHasher(str);
                        package.Identifiers[id] = str;
                        package.Tokens.push_back(gen::Token(gen::TokenType::Identifier, id));
                    }
                }

                // symbols
                else 
                {
                    if (index + 1 < buffer.size() && !isalnum(buffer[index + 1]))
                    {
                        tBuffer << c_char << buffer[index + 1];
                        const std::string c_2char = tBuffer.str();
                        tBuffer.str("");

                        if (gen::s_TokenMap.count(c_2char) > 0)
                        {
                            package.Tokens.push_back(gen::Token(gen::s_TokenMap[c_2char]));
                            index += 2;
                            continue;
                        }
                        else if (c_2char == "/*")
                        {
                            exclude =  true;
                            index += 2;
                            continue;
                        }
                        else if (c_2char == "//")
                        {
                            skipLine = true;
                            index += 2;
                            continue;
                        }
                    }

                    const std::string as_str = std::string(1, c_char);
                    if (gen::s_TokenMap.count(as_str) > 0)
                        package.Tokens.push_back(gen::Token(gen::s_TokenMap[as_str]));
                    // else if (c_char == '"')

                    ++index;
                }
            }
        }

        file.close();
        return package;
    }

}