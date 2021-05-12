struct Autosave {
    struct Format {
        virtual ~Format() = default;
        virtual void parse(std::string&) = 0;
        virtual void render(std::stringstream&) = 0;
        static int parseInt(std::string& value) {
            size_t end;
            int i = std::stoi(value, &end);
            value = value.substr(end + (end < value.size()));
            return i;
        }
    };

    template <typename T>
    struct Number : Format {
        T& data;
        Number(T& d) : data(d) {
        }
        void parse(std::string& value) override {
            data = static_cast<T>(Format::parseInt(value));
        }
        void render(std::stringstream& s) override {
            s << static_cast<int>(data);
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
        void parse(std::string& value) override {
            while (value.size()) {
                auto i = Format::parseInt(value);
                active(data[i]) = true;
                std::unique_ptr<Format>(format(data[i]))->parse(value);
            }
        }
        void render(std::stringstream& s) override {
            for (int i = 0; i < N; i++) {
                if (active(data[i])) {
                    s << i << "@";
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
            auto remainingContent = content.substr(newline + 1);
            if (bindings.count(label))
                bindings[label]->parse(value);
            content = remainingContent;
        }
    }

  private:
    std::filesystem::path filename;
    std::unordered_map<std::string, std::unique_ptr<Format>> bindings;
};
