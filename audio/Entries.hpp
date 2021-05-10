struct Entries : GenericUI {
    struct Entry {
        const char* label;
        float value;
        float min;
        float max;
        float step;
    };

    Entries() : data() {
    }

    void bind(const std::string& prefix, Autosave* autosave) {
        for (auto& entry : data)
            autosave->bind(prefix + entry.label, new Autosave::Number(entry.value));
    }

    /*
     * Since the order of calls to `addNumEntry` will be stable,
     * we can use a simple vector to power our dynamic voice assigmnent.
     * When `buildUserInterface` is called initially, we append each new entry
     * into the `data` vector. After a call to `prepareToWrite` however,
     * instead of pushing _to_ the array, we read _from_ it. On subsequent calls
     * to `buildUserInterface`, the calling dsp has its zones set to reflect
     * the values in `data`.
     */
    void addNumEntry(const char* label, FAUSTFLOAT* zone, FAUSTFLOAT init, FAUSTFLOAT min, FAUSTFLOAT max, FAUSTFLOAT step) override {
        if (writeIndex == -1)
            data.push_back({ label, init, min, max, step });
        else
            *zone = data[writeIndex++].value;
    }

    bool find(const std::string& name, Entries::Entry*& entry) {
        for (auto& e : data) {
            if (name == e.label) {
                entry = &e;
                return true;
            }
        }
        return false;
    }

    void prepareToWrite() {
        writeIndex = 0;
    }

  private:
    int writeIndex = -1;
    std::vector<Entry> data;
};
