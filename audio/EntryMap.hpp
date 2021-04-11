struct EntryMap : GenericUI {
    struct Entry {
        float value;
        float min;
        float max;
        float step;
    };

    std::unordered_map<std::string, Entry> contents;

    EntryMap() : contents() {
    }

    void addNumEntry(const char* label, FAUSTFLOAT* zone, FAUSTFLOAT init, FAUSTFLOAT min, FAUSTFLOAT max, FAUSTFLOAT step) override {
        contents[std::string(label)] = { init, min, max, step };
    }
};
