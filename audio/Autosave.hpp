struct Autosave {
    struct Format {
        virtual ~Format() = default;
        virtual void parse(std::string, size_t*) = 0;
        virtual void render(std::stringstream&) = 0;
    };

    template <typename T>
    struct Number : Format {
        T& data;
        Number(T& d) : data(d) {
        }
        void parse(std::string value, size_t* end) override {
            data = static_cast<T>(std::stoi(value, end));
        }
        void render(std::stringstream& s) override {
            s << std::to_string(static_cast<int>(data));
        }
    };

    template <typename T, size_t N>
    struct Array : Format {
        std::array<T, N>& data;
        std::function<bool&(T&)> active;
        std::function<Format*(T&)> format;
        template <typename A, typename F>
        Array(std::array<T, N>& d, A&& a, F&& f) : data(d), active(a), format(f) {
        }
        void parse(std::string value, size_t* end) override {
            while (value.size()) {
                auto i = std::stoi(value, end);
                value = value.substr(*end + 1);
                active(data[i]) = true;
                std::unique_ptr<Format>(format(data[i]))->parse(value, end);
                value = value.substr(*end + 1);
            }
        }
        void render(std::stringstream& s) override {
            for (int i = 0; i < N; i++) {
                if (active(data[i])) {
                    s << std::to_string(i) << "@";
                    std::unique_ptr<Format>(format(data[i]))->render(s);
                    s << ",";
                }
            }
        }
    };

    Autosave(std::filesystem::path f) : filename(f) {
    }

    void write() {
        std::stringstream content;
        for (const auto& binding : bindings) {
            content << binding.first << "=";
            binding.second->render(content);
            content << "\n";
        }
        choc::file::replaceFileWithContent(filename, content.str());
    }


    void bind(const std::string label, Format* format) {
        bindings[label] = std::unique_ptr<Format>(format);
    }

    void load() {
        if (!std::filesystem::exists(filename))
            return;
        auto content = choc::file::loadFileAsString(filename);
        while (content.size()) {
            auto equals = content.find('=');
            auto newline = content.find('\n');
            auto label = content.substr(0, equals);
            auto value = content.substr(equals + 1, newline - equals - 1);
            size_t end;
            if (bindings.count(label))
                bindings[label]->parse(value, &end);
            content = content.substr(newline + 1);
        }
    }

  private:
    std::filesystem::path filename;
    std::unordered_map<std::string, std::unique_ptr<Format>> bindings;
};
