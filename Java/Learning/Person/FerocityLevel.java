package Person;

public enum FerocityLevel {
    LOW,
    MEDIUM,
    HIGH,;
    public FerocityLevel default_val(){
        return FerocityLevel.MEDIUM;
    }
}
