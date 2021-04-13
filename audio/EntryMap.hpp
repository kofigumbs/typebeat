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
        auto s = std::string(label);
        if (s.find("~"))
            contents[s.substr(1)] = { init, min, max, step };
    }
};
