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
        if (label[0] == '~')
            contents[std::string(label).substr(1)] = { init, min, max, step };
    }
};
