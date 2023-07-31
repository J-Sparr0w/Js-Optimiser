import Animated, {
  Extrapolation,
  interpolate,
  useAnimatedScrollHandler,
  useAnimatedStyle,
  useSharedValue,
  withTiming,
} from "react-native-reanimated";
import NextButton from "../components/NextButton";

const ListItem = ({ item, index, activeIndex, scrollX }) => {
  // const isActive = index === activeIndex;

  const animatedStyles = useAnimatedStyle(() => {
    const inputRange = [
      (index - 2) * ITEM_WIDTH,
      (index - 1) * ITEM_WIDTH,
      index * ITEM_WIDTH,
    ];
    return {
      opacity: withTiming(
        interpolate(
          scrollX.value,
          inputRange,
          [0.4, 1, 0.4],
          Extrapolation.CLAMP
        )
      ),
      transform: [
        {
          scale: withTiming(
            interpolate(
              scrollX.value,
              inputRange,
              [0.75, 1.2, 0.75],
              Extrapolation.CLAMP
            )
          ),
        },
      ],
    };
  });

  const listItemStyles = StyleSheet.create({
    item: {
      width: ITEM_WIDTH,
      height: ITEM_HEIGHT,
      opacity: 1,
      justifyContent: "center",

      borderRadius: 50,
      // backgroundColor: "grey",
    },
    age: {
      alignSelf: "center",
      color: "black",
      fontSize: 35,
    },
  });

  return (
    <View style={listItemStyles.item}>
      <Animated.Text style={[listItemStyles.age, animatedStyles]}>
        {item}
      </Animated.Text>
    </View>
  );
};

export default function AgeSelectionScreen({ navigation }) {
  const maxAge = 120;
  const minAge = 20;
  const numArray = [...Array(50).keys()].map((v, i) => {
    return (v = i + minAge);
  });
  const [ageList, setAgeList] = useState(numArray);
  const scrollX = useSharedValue(0);

  const [selectedIndex, setSelectedIndex] = useState(1);

  //load next 10 numbers on reaching scroll-end
  const loadNum = () => {
    const startingNumToBeLoaded = ageList[ageList.length - 1] + 1;
    if (startingNumToBeLoaded > maxAge) return;

    const nextArr = [...Array(20).keys()].map((v, i) => {
      return (v = i + startingNumToBeLoaded);
    });
    setAgeList([...ageList, ...nextArr]);
  };

  const scrollHandler = useAnimatedScrollHandler({
    onScroll: (e) => {
      const offsetX = e.contentOffset.x;
      scrollX.value = offsetX;
    },
  });

  return (
    <View style={styles.container}>
      <QuestionHeader>
        Sarah, for personalized lifestyle recommendations, select your age
      </QuestionHeader>
      <View style={styles.bottomSection}>
        <View style={styles.ageListWrapper}>
          <Animated.FlatList
            horizontal={true}
            keyExtractor={(item) => item.toString()}
            data={ageList}
            showsHorizontalScrollIndicator={false}
            style={{ flexGrow: 0 }}
            initialNumToRender={15}
            getItemLayout={(data, index) => ({
              length: ITEM_WIDTH,
              offset: ITEM_WIDTH * index,
              index,
            })}
            onEndReached={(info) => {
              loadNum();
            }}
            initialScrollIndex={30}
            onScroll={scrollHandler}
            onMomentumScrollEnd={(e) => {
              const offsetX = e.nativeEvent.contentOffset.x;
              const index = Math.round(offsetX / ITEM_WIDTH) + 1;
              console.log("index  ", index);
              console.log("index val ", ageList[index]);
              setSelectedIndex(index);
            }}
            scrollEventThrottle={16}
            bounces={false}
            snapToInterval={ITEM_WIDTH}
            decelerationRate={"fast"}
            maxToRenderPerBatch={10}
            renderItem={({ item, index }) => (
              <ListItem
                item={item}
                index={index}
                activeIndex={selectedIndex}
                scrollX={scrollX}
              />
            )}
          />
        </View>

        <NextButton
          width={150}
          height={60}
          style={{ marginBottom: 100, borderRadius: 10 }}
          onPress={() => {
            navigation.navigate("Health");
          }}
        />
      </View>
    </View>
  );
}

const PADDING = 10;
const LIST_WRAPPER_WIDTH = 279;
const LIST_WRAPPER_HEIGHT = 80;
const ITEM_WIDTH = LIST_WRAPPER_WIDTH / 3;
const ITEM_HEIGHT = LIST_WRAPPER_HEIGHT;

const styles = StyleSheet.create({
  container: {
    flex: 1,
    padding: PADDING,
    paddingTop: StatusBar.currentHeight,

    backgroundColor: COLORS.OLIVE_GREEN,
  },
  bottomSection: {
    flex: 1,
    marginTop: 20,
    padding: PADDING,
    // backgroundColor: "grey",
    alignItems: "center",
    justifyContent: "space-between",
  },
  ageListWrapper: {
    width: LIST_WRAPPER_WIDTH,
    height: LIST_WRAPPER_HEIGHT,
    backgroundColor: "white",
    opacity: 0.7,
    borderRadius: 50,
  },
});
