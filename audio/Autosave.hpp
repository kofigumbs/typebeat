struct Autosave {
    struct Format {
        void* data;
        Format(void* d) : data(d) {
        }
        virtual ~Format() = default;
        virtual void parse(std::string, size_t* end) = 0;
        virtual std::string render() = 0;
    };

    template <typename T>
    struct Number : Format {
        Number(T& data) : Format(&data) {
        }
        void parse(std::string value, size_t* end) override {
            *(T*) data = static_cast<T>(std::stoi(value, end));
        }
        std::string render() override {
            return std::to_string(static_cast<int>(*(T*) data));
        }
    };

    template <typename T, typename M, size_t N>
    struct Array : Format {
        M T::* member;
        Array(std::array<T, N>& array, M T::* m) : Format(array.data()), member(m) {
        }
        void parse(std::string value, size_t* end) override {
            for (int i = 0; i < N && value.size(); i++) {
                format(i).parse(value, end);
                value = value.substr(*end + 1);
            }
        }
        std::string render() override {
            std::stringstream s;
            for (int i = 0; i < N; i++)
                s << format(i).render() << ",";
            return s.str();
        }
        Number<M> format(int i) {
            return Number(((T*) data)[i].*member);
        }
    };

    Autosave(std::filesystem::path f) : filename(f), writer(&Autosave::run, this) {
    }

    ~Autosave() {
        running = false;
        writer.join();
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
            auto value = content.substr(equals + 1, newline);
            size_t end;
            if (bindings.count(label))
                bindings[label]->parse(value, &end);
            content = content.substr(newline + 1);
        }
    }

    void save() {
        dirty = true;
    }

  private:
    bool running = true;
    std::filesystem::path filename;
    std::thread writer;
    std::atomic<bool> dirty;
    std::unordered_map<std::string, std::unique_ptr<Format>> bindings;

    void run() {
        while (running || dirty.load()) {
            if (dirty.exchange(false)) {
                std::stringstream content;
                for (const auto& binding : bindings)
                    content << binding.first << "=" << binding.second->render() << "\n";
                choc::file::replaceFileWithContent(filename, content.str());
            }
            std::this_thread::sleep_for(std::chrono::seconds(2));
        }
    }
};
