struct EntryMap : GenericUI {
    struct Entry {
        float value;
        float min;
        float max;
        float step;
    };

    bool skipNext = false;
    std::unordered_map<std::string, Entry> contents;

    EntryMap() : contents() {
    }

    void declare(FAUSTFLOAT* zone, const char* key, const char* value) override {
        if (std::string(key) == "hidden")
            skipNext = true;
    }

    void addNumEntry(const char* label, FAUSTFLOAT* zone, FAUSTFLOAT init, FAUSTFLOAT min, FAUSTFLOAT max, FAUSTFLOAT step) override {
        if (!skipNext)
            contents[label] = { init, min, max, step };
        skipNext = false;
    }
};
