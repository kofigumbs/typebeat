struct Autosave {
    enum class Type {
        Bool,
        Int,
        Float,
        Custom,
    };

    struct Custom {
        std::function<void(std::string)> parse;
        std::function<std::string()> render;
    };

    Autosave(std::filesystem::path f) : filename(f), writeThread(&Autosave::run, this) {
    }

    ~Autosave() {
        running = false;
        writeThread.join();
    }

    void bind(const std::string label, bool* data)  { bind(label, data, Type::Bool);  }
    void bind(const std::string label, int* data)   { bind(label, data, Type::Int);   }
    void bind(const std::string label, float* data) { bind(label, data, Type::Float); }
    void bind(const std::string label, Custom* c) {
        custom.emplace_back(c);
        bind(label, custom.back().get(), Type::Custom);
    }

    void load() {
        if (!std::filesystem::exists(filename))
            return;
        auto content = choc::file::loadFileAsString(filename);
        std::regex line("^([^ ]+) = ([^\n]+)\n");
        std::smatch match; 
        while(regex_search(content, match, line)) {
            const auto& label = match[1].str();
            const auto& value = match[2].str();
            if (bindings.count(label))
                parse(value, bindings[label]);
            content = match.suffix();
        }
    }

    void save() {
        shouldWrite = true;
    }

  private:
    bool running = true;
    std::filesystem::path filename;
    std::thread writeThread;
    std::atomic<bool> shouldWrite;
    std::vector<std::unique_ptr<Custom>> custom;
    std::unordered_map<std::string, std::pair<void*, Type>> bindings;

    void bind(const std::string label, void* data, Type type) {
        bindings[label] = { data, type };
    }

    void parse(const std::string& value, std::pair<void*, Type> binding) {
        switch (binding.second) {
            case Type::Bool:
                *(bool*) binding.first = value == "1";
                return;
            case Type::Int:
                *(int*) binding.first = std::stoi(value);
                return;
            case Type::Float:
                *(float*) binding.first = std::stoi(value);
                return;
            case Type::Custom:
                ((Custom*) binding.first)->parse(value);
                return;
        }
    }

    void run() {
        while (running) {
            if (shouldWrite.exchange(false)) {
                std::stringstream content;
                for (const auto& binding : bindings)
                    content << binding.first << " = " << render(binding.second) << std::endl;
                choc::file::replaceFileWithContent(filename, content.str());
            }
            std::this_thread::sleep_for(std::chrono::seconds(2));
        }
    }

    std::string render(const std::pair<void*, Type>& binding) {
        switch (binding.second) {
            case Type::Bool:
                return *(bool*) binding.first ? "1" : "0";
            case Type::Int:
                return std::to_string(*(int*) binding.first);
            case Type::Float:
                return std::to_string(int(*(float*) binding.first));
            case Type::Custom:
                return ((Custom*) binding.first)->render();
        }
    }
};
